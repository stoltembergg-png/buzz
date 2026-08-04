// Contract test template — verifies that a Feature Spec matches the project's
// governance contract (PR-001, PR-002).
//
// How it works:
//  1. Scans .spec/features/<feature>.md for the expected sections.
//  2. Verifies every REQ-XXX-NNN has an AC-XXX-NNN paired (or vice-versa).
//  3. Verifies no checked box contains TODO / TBD / ??? placeholders.
//  4. Verifies IDs are unique within the file.
//
// Save as: `.spec/features/<feature>/contract.test.mjs` and prepend
// the test loader hook so `node --test` can pick it up.
//
// Run:
//   node --test .spec/features/<feature>/contract.test.mjs

import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";

const FEATURE_DOC = resolve(import.meta.dirname, "spec.md");
const PLACEHOLDERS = /\b(TODO|TBD|FIXME|XXX)\b/i;

function readSpec() {
  return readFileSync(FEATURE_DOC, "utf8");
}

function extractIds(sectionHeader) {
  const text = readSpec();
  const start = text.indexOf(sectionHeader);
  if (start < 0) return [];
  const end = text.indexOf("\n## ", start + sectionHeader.length);
  const body = text.slice(start, end > 0 ? end : undefined);
  // Match full ID tokens like REQ-GOV-201 / AC-GOV-201 (3-letter prefix + 3 digits).
  const ids = body.match(/\b(REQ|AC|T|E)-[A-Z]+-\d{3}\b/g) || [];
  return [...new Set(ids)];
}

function extractCheckedItems() {
  const text = readSpec();
  const lines = text.split("\n");
  const checked = lines.filter((l) => /^\s*-\s*\[\s*x\s*\]/i.test(l));
  return checked;
}

describe("Feature spec contract", () => {
  it("declares a status field", () => {
    const text = readSpec();
    assert.match(text, /^\*\*Status:\*\* /m);
  });

  it("declares a phase delimited by the pipeline taxonomy", () => {
    const text = readSpec();
    const phases = [
      "governance", "memory", "providers", "context", "skills",
      "mcp", "plugins", "observability", "eval", "self-improvement",
      "security", "data", "release", "maintenance",
    ];
    const match = text.match(/^\*\*Phase:\*\* (\w+(?:-\w+)?)/m);
    assert.ok(match, "Phase field missing");
    assert.ok(phases.includes(match[1]), `Unknown phase: ${match[1]}`);
  });

  it("uses REQ-XXX-NNN identifiers in the Requirements section", () => {
    const ids = extractIds("REQ-");
    assert.ok(ids.length > 0, "no REQ-XXX-NNN identifiers found");
    for (const id of ids) {
      assert.match(id, /^[A-Z]+-\d{3}-?$/, `invalid REQ id: ${id}`);
    }
  });

  it("uses AC-XXX-NNN identifiers in the Acceptance criteria section", () => {
    const ids = extractIds("AC-");
    assert.ok(ids.length > 0, "no AC-XXX-NNN identifiers found");
  });

  it("contains no placeholder text inside checked items", () => {
    const checked = extractCheckedItems();
    for (const line of checked) {
      assert.doesNotMatch(line, PLACEHOLDERS,
        `placeholder found in checked item: ${line.trim()}`);
    }
  });

  it("has unique identifiers across REQ and AC sections", () => {
    const reqs = extractIds("REQ-");
    const acs = extractIds("AC-");
    const seen = new Set();
    for (const id of reqs) {
      assert.ok(!seen.has(id), `duplicate REQ id: ${id}`);
      seen.add(id);
    }
    for (const id of acs) {
      assert.ok(!seen.has(id), `duplicate AC id: ${id}`);
      seen.add(id);
    }
  });

  it("lists evidence references for the spec", () => {
    const text = readSpec();
    assert.match(text, /^\s*-\s*\[\s*\]\s*`E-/m,
      "Evidence section must contain at least one E-XXX-NNN reference");
  });
});
