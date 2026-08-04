#!/usr/bin/env bash
# verify-feature.sh — local feature verification gate (PR-003)
#
# Validates traceability between a feature spec and its contract tests:
#   .spec/features/<feature>/spec.md       ← requirements + ACs (AC-NNN)
#   .spec/features/<feature>/contract.test.mjs ← node:test cases tagged @spec:AC-NNN
#   .spec/verification/<feature>.json     ← evidence record (written on success)
#
# Exit codes (deterministic):
#   0  feature is valid and fully traced
#   1  feature directory not found
#   2  spec.md or contract.test.mjs missing
#   3  duplicate AC IDs in spec
#   4  AC in spec has no matching test (orphan requirement)
#   5  test references AC with no matching spec entry (orphan test)
#   6  task marked done without evidence
#   7  contract tests failed (node --test exit ≠ 0)
#   8  feature path contains traversal or unsafe characters
#
# Usage:
#   scripts/verify-feature.sh <feature> [--json]
#
set -euo pipefail

FEATURE="${1:-}"
OUTPUT_JSON="${2:-}"

if [[ -z "$FEATURE" ]]; then
  echo "Usage: $0 <feature> [--json]" >&2
  exit 1
fi

# ----- Security: reject path traversal and unsafe characters -----
# REQ-GOV-306 — don't interpret spec content as shell; don't allow escaping .spec/features/
if [[ "$FEATURE" =~ [^a-zA-Z0-9._-] ]] || [[ "$FEATURE" == ".."* ]] || [[ "$FEATURE" == *".."* ]] || [[ "$FEATURE" == *"/"* ]]; then
  echo "ERROR: feature name contains unsafe characters or traversal: '$FEATURE'" >&2
  exit 8
fi

cd "$(dirname "$0")/.." && REPO_ROOT="$(pwd)"
FEATURE_DIR="$REPO_ROOT/.spec/features/$FEATURE"
SPEC_FILE="$FEATURE_DIR/spec.md"
CONTRACT_FILE="$FEATURE_DIR/contract.test.mjs"

# ----- REQ-GOV-301: feature directory must exist -----
if [[ ! -d "$FEATURE_DIR" ]]; then
  echo "ERROR: feature directory not found: $FEATURE_DIR" >&2
  exit 1
fi

# ----- REQ-GOV-302: spec.md and contract.test.mjs must exist -----
if [[ ! -f "$SPEC_FILE" ]]; then
  echo "ERROR: spec.md missing for feature '$FEATURE'" >&2
  exit 2
fi
if [[ ! -f "$CONTRACT_FILE" ]]; then
  echo "ERROR: contract.test.mjs missing for feature '$FEATURE'" >&2
  exit 2
fi

# ----- Extract AC IDs from spec.md -----
# Accepts AC-NNN (short, 3 digits) or AC-XXX-NNN (with domain prefix, e.g. AC-GOV-201)
SPEC_ACS_ALL=$(grep -oE '\bAC-([A-Z]+-)?[0-9]{3}\b' "$SPEC_FILE" || true)
SPEC_ACS=$(echo "$SPEC_ACS_ALL" | sort -u || true)
SPEC_AC_COUNT=$(echo "$SPEC_ACS" | grep -c . || true)

# ----- Check duplicates in spec (before unique) -----
SPEC_DUPS=$(echo "$SPEC_ACS_ALL" | sort | uniq -d || true)
if [[ -n "$SPEC_DUPS" ]]; then
  echo "ERROR: duplicate AC IDs in spec: $SPEC_DUPS" >&2
  exit 3
fi

# ----- Extract @spec:AC-NNN or @spec:AC-XXX-NNN tags from contract.test.mjs -----
# Use a Node.js helper script to extract @spec:AC tags.
# Persisted at .spec/templates/extract-spec-tags.cjs.
# Convert POSIX paths to Windows so node can find the files.
EXTRACTOR_WIN=$(cygpath -w "$REPO_ROOT/.spec/templates/extract-spec-tags.cjs" 2>/dev/null || echo "$REPO_ROOT/.spec/templates/extract-spec-tags.cjs")
CONTRACT_WIN=$(cygpath -w "$CONTRACT_FILE" 2>/dev/null || echo "$CONTRACT_FILE")
TEST_TAGS=$(node "$EXTRACTOR_WIN" "$CONTRACT_WIN" 2>/dev/null || echo "")
TEST_TAG_COUNT=$(echo "$TEST_TAGS" | grep -c . || true)

# Note: duplicate @spec:AC tags in tests are allowed (multiple tests per AC)

# ----- REQ-GOV-302: every AC in spec must have a matching test (orphan requirement) -----
ORPHAN_REQS=""
for ac in $SPEC_ACS; do
  if ! echo "$TEST_TAGS" | grep -qx "$ac"; then
    ORPHAN_REQS="$ORPHAN_REQS $ac"
  fi
done
if [[ -n "$ORPHAN_REQS" ]]; then
  echo "ERROR: AC(s) in spec have no test: $ORPHAN_REQS" >&2
  exit 4
fi

# ----- REQ-GOV-303: every test tag must reference a spec AC (orphan test) -----
ORPHAN_TESTS=""
for tt in $TEST_TAGS; do
  if ! echo "$SPEC_ACS" | grep -qx "$tt"; then
    ORPHAN_TESTS="$ORPHAN_TESTS $tt"
  fi
done
if [[ -n "$ORPHAN_TESTS" ]]; then
  echo "ERROR: test tags reference unknown AC(s): $ORPHAN_TESTS" >&2
  exit 5
fi

# ----- REQ-GOV-304: tasks marked done must have evidence -----
TASKS_FILE="$FEATURE_DIR/tasks.md"
if [[ -f "$TASKS_FILE" ]]; then
  DONE_TASKS=$(grep -E '^\|\s*T-[0-9A-Z-]+\s*\|.*\|\s*done\s*\|' "$TASKS_FILE" || true)
  if [[ -n "$DONE_TASKS" ]]; then
    for line in $DONE_TASKS; do
      EVIDENCE_REF=$(echo "$line" | grep -oE 'E-[A-Z]+-[0-9]{3}' || true)
      if [[ -z "$EVIDENCE_REF" ]]; then
        echo "ERROR: task marked done without evidence ref: $line" >&2
        exit 6
      fi
    done
  fi
fi

# ----- Run the contract tests (REQ-GOV-305: record SHA + platform) -----
SHA=$(cd "$REPO_ROOT" && git rev-parse --short HEAD 2>/dev/null || echo "no-git")
PLATFORM_OS=$(uname -s 2>/dev/null || echo "unknown")
PLATFORM_ARCH=$(uname -m 2>/dev/null || echo "unknown")
NODE_VERSION=$(node --version 2>/dev/null || echo "no-node")

REPORT_DIR=$(mktemp -d)
trap 'rm -rf "$REPORT_DIR"' EXIT

node --test --test-reporter=tap "$CONTRACT_FILE" > "$REPORT_DIR/tap.txt" 2>&1 || true
NODE_EXIT=$?

PASSED_AC_TAGS=$(grep -E '^# (Subtest:|ok) .*@spec:AC-' "$REPORT_DIR/tap.txt" | grep '^# ok\|^ok' | grep -oE '@spec:AC-([A-Z]+-)?[0-9]{3}' | sed 's/@spec://' | sort -u || true)
FAILED_AC_TAGS=$(grep -E '^# (Subtest:|not ok) .*@spec:AC-' "$REPORT_DIR/tap.txt" | grep '^# not ok\|^not ok' | grep -oE '@spec:AC-([A-Z]+-)?[0-9]{3}' | sed 's/@spec://' | sort -u || true)

if [[ "$NODE_EXIT" -ne 0 ]] || [[ -n "$FAILED_AC_TAGS" ]]; then
  echo "ERROR: contract tests failed. Failed ACs: $FAILED_AC_TAGS" >&2
  echo "--- TAP output ---" >&2
  cat "$REPORT_DIR/tap.txt" >&2 || true
  exit 7
fi

# ----- Build evidence record -----
if [[ "$OUTPUT_JSON" == "--json" ]]; then
  cat <<EOF
{
  "feature": "$FEATURE",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "gitRev": "$SHA",
  "platform": { "os": "$PLATFORM_OS", "arch": "$PLATFORM_ARCH" },
  "tools": { "node": "$NODE_VERSION" },
  "command": "node --test $CONTRACT_FILE",
  "exitCode": 0,
  "specACs": $(echo "$SPEC_ACS" | node -e "const a=require('fs').readFileSync(0,'utf8').split(/\\s+/).filter(x=>x);console.log(JSON.stringify(a))" 2>/dev/null || echo "[]"),
  "testACs": $(echo "$TEST_TAGS" | node -e "const a=require('fs').readFileSync(0,'utf8').split(/\\s+/).filter(x=>x);console.log(JSON.stringify(a))" 2>/dev/null || echo "[]"),
  "passedACs": $(echo "$PASSED_AC_TAGS" | node -e "const a=require('fs').readFileSync(0,'utf8').split(/\\s+/).filter(x=>x);console.log(JSON.stringify(a))" 2>/dev/null || echo "[]"),
  "results": { "totalSpecACs": $SPEC_AC_COUNT, "totalTestACs": $TEST_TAG_COUNT }
}
EOF
else
  echo "✓ Feature '$FEATURE' verified successfully."
  echo "  Spec ACs:   $SPEC_AC_COUNT"
  echo "  Test ACs:   $TEST_TAG_COUNT"
  echo "  SHA:        $SHA"
  echo "  Platform:   $PLATFORM_OS/$PLATFORM_ARCH"
  echo "  Node:       $NODE_VERSION"
  echo "  Contract:   $CONTRACT_FILE"
fi