#!/usr/bin/env node
// extract-spec-tags.cjs — extract @spec:AC-* tags from contract.test.mjs
//
// Strategy: replace string LITERAL contents with spaces of equal length,
// then run regex over the cleaned code. This way @spec:AC- tags that
// appear ONLY inside string literals (e.g. inside fixture data passed as
// an argument) are not counted — but tags that are part of the test
// description string "test('@spec:AC-301 foo')" ARE preserved because
// the test() call itself is in code, only the string contents get blanked.
//
// Wait — that loses the description. Better strategy: find test()/it()
// calls in code, then look at the corresponding string literals in the
// ORIGINAL text.

const fs = require("fs");
const path = process.argv[2];
if (!path) {
  console.error("usage: extract-spec-tags.cjs <contract.test.mjs>");
  process.exit(2);
}
const text = fs.readFileSync(path, "utf8");

// Find all test(...) or it(...) calls in code (not inside strings).
// We use a state-machine tokenizer that tracks string-literal state.
function findTestDescs(src) {
  const descs = [];
  let i = 0;
  const n = src.length;
  while (i < n) {
    const c = src[i];

    // Skip string literals (track but don't descend into them)
    if (c === '"' || c === "'" || c === "`") {
      const q = c;
      let j = i + 1;
      while (j < n) {
        if (src[j] === "\\") { j += 2; continue; }
        if (src[j] === q) { j++; break; }
        j++;
      }
      i = j;
      continue;
    }

    // Skip comments
    if (c === "/" && src[i + 1] === "/") {
      const end = src.indexOf("\n", i);
      i = end < 0 ? n : end;
      continue;
    }
    if (c === "/" && src[i + 1] === "*") {
      const end = src.indexOf("*/", i + 2);
      i = end < 0 ? n : end + 2;
      continue;
    }

    // Match test( or it(
    if ((src.startsWith("test", i) || src.startsWith("it", i)) &&
        src[i + 4] === "(" || (src.startsWith("test", i) || src.startsWith("it", i)) && src[i + 2] === "(") {
      // Skip test or it keyword
      const kwLen = src[i + 2] === "(" ? 2 : 4;
      // Skip whitespace
      let j = i + kwLen;
      while (j < n && /\s/.test(src[j])) j++;
      // Expect (
      if (src[j] !== "(") { i++; continue; }
      j++;
      // Skip whitespace
      while (j < n && /\s/.test(src[j])) j++;
      // First arg must be string literal "..." or '...' or `...`
      const quote = src[j];
      if (quote !== '"' && quote !== "'" && quote !== "`") { i = j; continue; }
      j++;
      // Read until matching quote
      let str = "";
      while (j < n) {
        if (src[j] === "\\") { str += src[j] + (src[j + 1] || ""); j += 2; continue; }
        if (src[j] === quote) break;
        str += src[j];
        j++;
      }
      descs.push(str);
      i = j + 1;
      continue;
    }

    i++;
  }
  return descs;
}

const descs = findTestDescs(text);
const tags = [];
for (const d of descs) {
  for (const m of d.matchAll(/@spec:(AC-(?:[A-Z]+-)?\d{3})/g)) {
    tags.push(m[1]);
  }
}
const uniq = [...new Set(tags)].sort();
console.log(uniq.join("\n"));