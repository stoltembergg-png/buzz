import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "../../../",
);
const desktopManifest = path.join(
  repoRoot,
  "desktop/src-tauri/Cargo.toml",
);

function runCargo(args) {
  assert.ok(existsSync(path.join(repoRoot, "Cargo.toml")));
  return execFileSync("cargo", args, {
    cwd: repoRoot,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
}

function runDesktopRust(filter) {
  return runCargo([
    "test",
    "--manifest-path",
    desktopManifest,
    "--lib",
    filter,
  ]);
}

test("@spec:AC-001 Hermes runtime identity resolves all entrypoints", () => {
  runDesktopRust(
    "hermes_is_a_known_runtime_for_all_supported_entrypoints",
  );
});

test("@spec:AC-002 Hermes metadata declares the ACP host contract", () => {
  runDesktopRust(
    "hermes_is_a_known_runtime_for_all_supported_entrypoints",
  );
});

test("@spec:AC-003 Hermes config extracts safe provider/model forms", () => {
  runDesktopRust("managed_agents::config_bridge::hermes::tests::parses_");
});

test("@spec:AC-004 Hermes config honors HERMES_HOME", () => {
  runDesktopRust("managed_agents::config_bridge::hermes::tests::config_path_");
});

test("@spec:AC-005 Hermes config never surfaces credentials", () => {
  runDesktopRust(
    "managed_agents::config_bridge::hermes::tests::credential_fields_are_not_surfaceable",
  );
});

test("@spec:AC-006 Hermes has one catalog entry and a working ACP fallback", () => {
  runDesktopRust(
    "hermes_is_not_registered_as_a_second_layer_preset",
  );
  runDesktopRust(
    "hermes_cli_fallback_gets_acp_subcommand",
  );
  runCargo(["test", "-p", "buzz-acp", "normalizes_hermes_args_to_acp"]);
});
