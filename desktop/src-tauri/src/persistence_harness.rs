//! Memory persistence harness (PR-008)
//!
//! Integration test harness that proves memory persists across restarts
//! in the same scope and does not leak between scopes. Expresses the
//! end-to-end flow: compute scope → bootstrap profile → "write" marker →
//! restart → recover marker → assert different scope has no marker.
//!
//! This module intentionally does not run a real Hermes binary — it uses
//! the same library code paths (scope + bootstrap + env overlay) and a
//! marker file written under the scoped profile dir to simulate memory
//! contents. The harness is deterministic, side-effect-free outside its
//! TempDir, and cleans up after itself.
//!
//! Spec: docs/roadmap/prs/PR-008-memory-persistence-harness.md

use std::fs;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

use crate::hermes_home_inject::{compute_hermes_env_overlay, apply_overlay, EnvOverlay};
use crate::memory_scope::HermesMemoryScope;
use crate::runtime_scope::{RuntimeKind, RuntimeScopeContext};

/// Schema version for the harness report JSON (REQ-MEM-805).
pub const REPORT_SCHEMA_VERSION: u32 = 1;

/// A single scenario result row in the harness report.
#[derive(Debug, Clone)]
pub struct ScenarioResult {
    pub name: String,
    pub relay_url: String,
    pub channel_id: String,
    pub persona_id: String,
    pub runtime: RuntimeKind,
    pub marker_recovered: Option<String>,
    pub expected_recover: bool,
    pub passed: bool,
}

/// Harness report (REQ-MEM-805: SHA, Hermes, OS, commands, results).
#[derive(Debug, Clone)]
pub struct HarnessReport {
    pub schema: u32,
    pub timestamp: String,
    pub os: String,
    pub arch: String,
    pub scenarios: Vec<ScenarioResult>,
    pub all_passed: bool,
}

impl HarnessReport {
    /// Serialize to a minimal JSON string (no third-party serde needed; the
    /// report never carries secrets — REQ-MEM-806).
    pub fn to_json(&self) -> String {
        let mut s = String::new();
        s.push_str("{\n");
        s.push_str(&format!("  \"schema\": {},\n", self.schema));
        s.push_str(&format!("  \"timestamp\": \"{}\",\n", escape_json(&self.timestamp)));
        s.push_str(&format!("  \"os\": \"{}\",\n", escape_json(&self.os)));
        s.push_str(&format!("  \"arch\": \"{}\",\n", escape_json(&self.arch)));
        s.push_str("  \"scenarios\": [\n");
        for (i, sc) in self.scenarios.iter().enumerate() {
            s.push_str("    {\n");
            s.push_str(&format!("      \"name\": \"{}\",\n", escape_json(&sc.name)));
            s.push_str(&format!("      \"relay_url\": \"{}\",\n", escape_json(&sc.relay_url)));
            s.push_str(&format!("      \"channel_id\": \"{}\",\n", escape_json(&sc.channel_id)));
            s.push_str(&format!("      \"persona_id\": \"{}\",\n", escape_json(&sc.persona_id)));
            s.push_str(&format!("      \"runtime\": \"{}\",\n", sc.runtime));
            s.push_str(&format!("      \"marker_recovered\": {},\n",
                sc.marker_recovered.as_ref().map(|m| format!("\"{}\"", escape_json(m))).unwrap_or_else(|| "null".to_string())));
            s.push_str(&format!("      \"expected_recover\": {},\n", sc.expected_recover));
            s.push_str(&format!("      \"passed\": {}\n", sc.passed));
            s.push_str(if i + 1 == self.scenarios.len() { "    }\n" } else { "    },\n" });
        }
        s.push_str("  ],\n");
        s.push_str(&format!("  \"all_passed\": {}\n", self.all_passed));
        s.push_str("}\n");
        s
    }
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Run the persistence harness against a temp directory.
///
/// Returns a [`HarnessReport`] with per-scenario results and an overall
/// `all_passed` flag. The report does not contain full memory contents
/// (REQ-MEM-806) — only the marker digest prefix (first 8 hex chars).
pub fn run_harness() -> HarnessReport {
    let tmp = TempDir::new().expect("tempdir");
    let app_data = tmp.path();

    // Marker that simulates a unique memory write (REQ-MEM-802).
    let marker = format!("marker-{:016x}", rand_u64());

    // Scope A — used for write + restart-recovery scenarios.
    let ctx_a = RuntimeScopeContext::new(
        "wss://relay-a.example.com",
        "channel-a",
        "persona-a",
        RuntimeKind::Hermes,
    )
    .unwrap();

    // Variant scopes — used to prove isolation (REQ-MEM-804).
    let ctx_other_channel = RuntimeScopeContext::new(
        "wss://relay-a.example.com",
        "channel-b",
        "persona-a",
        RuntimeKind::Hermes,
    )
    .unwrap();
    let ctx_other_persona = RuntimeScopeContext::new(
        "wss://relay-a.example.com",
        "channel-a",
        "persona-b",
        RuntimeKind::Hermes,
    )
    .unwrap();
    let ctx_other_relay = RuntimeScopeContext::new(
        "wss://relay-b.example.com",
        "channel-a",
        "persona-a",
        RuntimeKind::Hermes,
    )
    .unwrap();

    let scenarios = vec![
        // REQ-MEM-802: write marker under scope A.
        Scenario::Write(ctx_a.clone(), marker.clone()),
        // REQ-MEM-803: restart in same scope recovers marker.
        Scenario::Restart(ctx_a.clone(), true),
        // REQ-MEM-804: different channel does not recover.
        Scenario::Restart(ctx_other_channel.clone(), false),
        // REQ-MEM-804: different persona does not recover.
        Scenario::Restart(ctx_other_persona.clone(), false),
        // REQ-MEM-804: different relay does not recover.
        Scenario::Restart(ctx_other_relay.clone(), false),
    ];

    let mut results = Vec::new();
    for sc in &scenarios {
        let result = run_scenario(sc, app_data);
        results.push(result);
    }

    let all_passed = results.iter().all(|r| r.passed);

    HarnessReport {
        schema: REPORT_SCHEMA_VERSION,
        timestamp: now_iso(),
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        scenarios: results,
        all_passed,
    }
}

enum Scenario {
    Write(RuntimeScopeContext, String),
    Restart(RuntimeScopeContext, bool),
}

fn run_scenario(sc: &Scenario, app_data: &Path) -> ScenarioResult {
    let (name, ctx, expected_recover) = match sc {
        Scenario::Write(ctx, _) => ("first write".to_string(), ctx.clone(), false),
        Scenario::Restart(ctx, expected) => ("restart recover".to_string(), ctx.clone(), *expected),
    };

    // Bootstrap + env overlay (the same code path the spawn would use).
    let overlay = compute_hermes_env_overlay(&ctx, app_data, None)
        .expect("overlay");

    let marker_path = match &overlay {
        Some(o) => o.value.join(".persistence-marker"),
        None => panic!("harness expects Hermes overlay"),
    };

    let marker_recovered = match sc {
        Scenario::Write(_, marker) => {
            // REQ-MEM-802: write the marker file under the scoped profile dir.
            fs::create_dir_all(&marker_path.parent().unwrap()).unwrap();
            fs::write(&marker_path, marker.as_bytes()).unwrap();
            // First write is not a "recovery".
            None
        }
        Scenario::Restart(_, _) => {
            // REQ-MEM-803: read marker if present.
            if marker_path.exists() {
                let content = fs::read_to_string(&marker_path).unwrap_or_default();
                Some(content)
            } else {
                None
            }
        }
    };

    // Pass criterion: matches expectation per scenario.
    let passed = match sc {
        Scenario::Write(_, _) => true, // write always "passes" if it reaches here
        Scenario::Restart(_, expected) => marker_recovered.is_some() == *expected,
    };

    let display = marker_recovered.as_ref().map(|m| {
        // REQ-MEM-806: never print full contents — show first 8 chars only.
        if m.len() > 8 { m[..8].to_string() } else { m.clone() }
    });

    ScenarioResult {
        name,
        relay_url: ctx.relay_url.clone(),
        channel_id: ctx.channel_id.clone(),
        persona_id: ctx.persona_id.clone(),
        runtime: ctx.runtime,
        marker_recovered: display,
        expected_recover,
        passed,
    }
}

fn now_iso() -> String {
    // Real ISO-8601 requires time crate; we use a stable unix approximation
    // for the report's timestamp. The exact wall clock is not asserted on.
    use std::time::{SystemTime, UNIX_EPOCH};
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{}s", dur.as_secs())
}

fn rand_u64() -> u64 {
    // Simple pseudo-random seed from thread + time.
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut h = DefaultHasher::new();
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    t.hash(&mut h);
    std::thread::current().id().hash(&mut h);
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    // REQ-MEM-801: single command runs the full scenario
    #[test]
    fn test_harness_runs_all_scenarios() {
        let report = run_harness();
        assert_eq!(report.schema, 1);
        assert_eq!(report.scenarios.len(), 5, "should run 5 scenarios");
    }

    // REQ-MEM-803: same scope restart recovers marker
    #[test]
    fn test_restart_same_scope_recovers_marker() {
        let report = run_harness();
        // The "restart recover" scenarios in order:
        // index 1 = same scope (channel-a, persona-a, relay-a) → expect true
        let same = &report.scenarios[1];
        assert!(same.marker_recovered.is_some(), "marker should be recovered");
        assert!(same.passed, "restart in same scope should pass");
    }

    // REQ-MEM-804: different channel does not recover
    #[test]
    fn test_different_channel_no_recovery() {
        let report = run_harness();
        let ch = &report.scenarios[2];
        assert!(ch.marker_recovered.is_none(), "different channel should NOT recover");
        assert!(ch.passed, "isolation test should pass (no recovery as expected)");
        assert_ne!(ch.channel_id, "channel-a");
    }

    // REQ-MEM-804: different persona does not recover
    #[test]
    fn test_different_persona_no_recovery() {
        let report = run_harness();
        let p = &report.scenarios[3];
        assert!(p.marker_recovered.is_none());
        assert!(p.passed);
        assert_ne!(p.persona_id, "persona-a");
    }

    // REQ-MEM-804: different relay does not recover
    #[test]
    fn test_different_relay_no_recovery() {
        let report = run_harness();
        let r = &report.scenarios[4];
        assert!(r.marker_recovered.is_none());
        assert!(r.passed);
        assert_ne!(r.relay_url, "wss://relay-a.example.com");
    }

    // REQ-MEM-805: report contains schema, OS, arch, scenarios
    #[test]
    fn test_report_has_required_fields() {
        let report = run_harness();
        let json = report.to_json();
        assert!(json.contains("\"schema\": 1"));
        assert!(json.contains("\"os\":"));
        assert!(json.contains("\"arch\":"));
        assert!(json.contains("\"scenarios\":"));
        assert!(json.contains("\"all_passed\":"));
    }

    // REQ-MEM-806: secrets/private content not printed in full
    #[test]
    fn test_report_redacts_marker_content() {
        let report = run_harness();
        // For scenarios where marker_recovered is Some, only first 8 chars
        // (REQ-MEM-806). The original marker is 25 chars ("marker-" + 16 hex).
        for sc in &report.scenarios {
            if let Some(m) = &sc.marker_recovered {
                assert!(m.len() <= 8, "marker leaked: {}", m);
            }
        }
    }

    // Overall: all scenarios pass
    #[test]
    fn test_all_scenarios_pass() {
        let report = run_harness();
        assert!(report.all_passed, "all scenarios must pass\n{}", report.to_json());
    }

    // Report JSON is valid JSON (no panic on parse)
    #[test]
    fn test_report_json_is_parseable() {
        let report = run_harness();
        let json = report.to_json();
        // We don't pull serde_json here — a minimal check that braces match.
        let open = json.matches('{').count();
        let close = json.matches('}').count();
        assert_eq!(open, close, "unbalanced braces in JSON report");
    }
}
