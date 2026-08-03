import type { BakedEnvEntry } from "@/shared/api/tauri";
import type { GlobalAgentConfig } from "@/shared/api/types";
import { getProviderEffortConfig } from "./buzzAgentConfig";

type AgentEffortStateInput = {
  bakedEnv: readonly BakedEnvEntry[];
  config: GlobalAgentConfig;
  effortFieldPresent: boolean;
  effortPersistenceKey: string | null;
  providerFieldVisible: boolean;
  reasoningCapabilityKnown: boolean;
  reasoningEfforts: readonly string[];
  selectedRuntimeId: string;
  showEffortField: boolean;
  usesNativeAcpEffort: boolean;
};

export function withoutEffortEnvVar(
  envVars: Record<string, string>,
  effortPersistenceKey: string | null,
): Record<string, string> {
  const effortKey = effortPersistenceKey ?? "BUZZ_AGENT_THINKING_EFFORT";
  return Object.fromEntries(
    Object.entries(envVars).filter(([key]) => key !== effortKey),
  );
}

export function resolveAgentEffortState({
  bakedEnv,
  config,
  effortFieldPresent,
  effortPersistenceKey,
  providerFieldVisible,
  reasoningCapabilityKnown,
  reasoningEfforts,
  selectedRuntimeId,
  showEffortField,
  usesNativeAcpEffort,
}: AgentEffortStateInput) {
  const nativeEffortDiscoveryReady =
    !usesNativeAcpEffort || reasoningCapabilityKnown;
  const effortProvider = providerFieldVisible
    ? (config.provider ?? "")
    : selectedRuntimeId === "claude"
      ? "anthropic"
      : selectedRuntimeId === "codex"
        ? "openai"
        : "";
  const { validValues: staticEffortValid, defaultValue: staticEffortDefault } =
    getProviderEffortConfig(effortProvider, config.model ?? "");
  const effortValid = usesNativeAcpEffort
    ? reasoningEfforts
    : staticEffortValid;
  const effortDefault = usesNativeAcpEffort
    ? reasoningEfforts.includes("medium")
      ? "medium"
      : (reasoningEfforts[0] ?? null)
    : staticEffortDefault;
  const currentEffortForAutoClear = effortPersistenceKey
    ? (config.env_vars[effortPersistenceKey] ?? "")
    : "";
  const bakedEffort = effortPersistenceKey
    ? (bakedEnv.find((entry) => entry.key === effortPersistenceKey)?.value ??
      null)
    : null;

  return {
    bakedEffort,
    currentEffortForAutoClear,
    effortDefault,
    effortFieldVisible:
      showEffortField &&
      effortFieldPresent &&
      nativeEffortDiscoveryReady &&
      (!usesNativeAcpEffort || reasoningEfforts.length > 0),
    effortValid,
    effortValidForAutoClear: effortValid,
    nativeEffortDiscoveryReady,
  };
}
