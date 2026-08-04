// Contract test for `.spec/features/pr-002-templates/spec.md`.
// Mirrors the template in `.spec/templates/contract.test.mjs` and asserts that
// the populated example satisfies the PR-001 / PR-002 governance contract.

import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const SPEC = resolve(import.meta.dirname, "spec.md");
const PLACEHOLDERS = /\b(TODO|TBD|FIXME|XXX)\b/;

function readSpec() { return readFileSync(SPEC, "utf8"); }

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
  return readSpec().split("\n").filter((l) => /^\s*-\s*\[\s*x\s*\]/i.test(l));
}

describe("PR-002 governance contract", () => {
  it("@spec:AC-GOV-201 RS-001 declares Status, Phase, Depends on", () => {
    const text = readSpec();
    assert.match(text, /^\*\*Status:\*\* done$/m);
    assert.match(text, /^\*\*Phase:\*\* governance$/m);
    assert.match(text, /^\*\*Depends on:\*\* PR-001$/m);
  });

  it("@spec:AC-GOV-202 RS-002 exposes REQ-XXX-NNN identifiers", () => {
    const ids = extractIds("REQ-");
    assert.deepEqual(ids, ["REQ-GOV-201", "REQ-GOV-202", "REQ-GOV-203", "REQ-GOV-204"],
      `unexpected REQ ids: ${ids.join(", ")}`);
  });

  it("@spec:AC-GOV-202 RS-003 exposes AC-XXX-NNN identifiers", () => {
    const ids = extractIds("AC-");
    assert.deepEqual(ids, ["AC-GOV-201", "AC-GOV-202", "AC-GOV-203"],
      `unexpected AC ids: ${ids.join(", ")}`);
  });

  it("@spec:AC-GOV-201 RS-004 contains no unchecked placeholders in checked items", () => {
    const checked = extractCheckedItems();
    for (const line of checked) {
      assert.doesNotMatch(line, PLACEHOLDERS,
        `placeholder found in checked item: ${line.trim()}`);
    }
  });

  it("@spec:AC-GOV-202 RS-005 has unique identifiers across REQ and AC sections", () => {
    const seen = new Set();
    for (const id of extractIds("REQ-")) {
      assert.ok(!seen.has(id), `duplicate id: ${id}`);
      seen.add(id);
    }
    for (const id of extractIds("AC-")) {
      assert.ok(!seen.has(id), `duplicate id: ${id}`);
      seen.add(id);
    }
  });

  it("@spec:AC-GOV-203 RS-006 references evidence in the Evidence section", () => {
    const text = readSpec();
    assert.match(text, /^\s*-\s*\[\s*\]\s*`E-/m,
      "evidence section must contain at least one E-XXX-NNN reference");
  });

  it("@spec:AC-GOV-203 RS-007 differentiation between automated tests and manual verification", () => {
    const text = readSpec();
    assert.match(text, /Tests\b[\s\S]*`T-GOV-/m);
  });

  it("@spec:AC-GOV-201 RS-008 all contract.test.mjs fields map to the PR-002 requirements", () => {
    const reqs = extractIds("REQ-");
    const text = readSpec();
    assert.ok(reqs.length > 0, "no REQ ids");
    assert.match(text, /\.spec\/features\/pr-002-templates\/contract\.test\.mjs/);
  });
});
