import type { ChannelMember } from "@/shared/api/types";

export type ChannelAgentIdentityMember = Pick<
  ChannelMember,
  "pubkey" | "displayName" | "isAgent" | "role"
>;

export type PersonaIdentity = {
  id: string;
  displayName: string;
};

export type ManagedAgentIdentity = {
  pubkey: string;
  personaId?: string | null;
};

/**
 * Agent labels are channel-scoped identities. Two different pubkeys may
 * represent two owners, but a shared channel must not silently acquire two
 * agents called "Fizz" or "Honey".
 */
export function normalizeChannelAgentLabel(
  label: string | null | undefined,
): string | null {
  const normalized = label?.normalize("NFKC").trim().toLowerCase() ?? "";
  return normalized.length > 0 ? normalized : null;
}

export function isChannelAgentMember(
  member: ChannelAgentIdentityMember,
): boolean {
  return member.isAgent === true || member.role === "bot";
}

export function getChannelAgentLabels(
  members: readonly ChannelAgentIdentityMember[],
): ReadonlySet<string> {
  const labels = new Set<string>();
  for (const member of members) {
    if (!isChannelAgentMember(member)) continue;
    const label = normalizeChannelAgentLabel(member.displayName);
    if (label) labels.add(label);
  }
  return labels;
}

export function isDuplicateChannelAgentMember(
  agent: { pubkey: string; name: string | null | undefined },
  members: readonly ChannelAgentIdentityMember[],
): boolean {
  const normalizedPubkey = agent.pubkey.trim().toLowerCase();
  if (
    members.some(
      (member) => member.pubkey.trim().toLowerCase() === normalizedPubkey,
    )
  ) {
    return false;
  }

  const label = normalizeChannelAgentLabel(agent.name);
  return label !== null && getChannelAgentLabels(members).has(label);
}

export function hasChannelAgentLabel(
  members: readonly ChannelAgentIdentityMember[],
  label: string | null | undefined,
): boolean {
  const normalized = normalizeChannelAgentLabel(label);
  return normalized !== null && getChannelAgentLabels(members).has(normalized);
}

/**
 * Returns persona IDs that cannot be added to a channel. The pubkey check
 * covers local managed agents; the label check covers agents owned by another
 * member whose persona metadata is not present in this worktree.
 */
export function getInChannelPersonaIds({
  members,
  managedAgents,
  personas,
}: {
  members: readonly ChannelAgentIdentityMember[];
  managedAgents: readonly ManagedAgentIdentity[];
  personas: readonly PersonaIdentity[];
}): ReadonlySet<string> {
  const memberPubkeys = new Set(
    members
      .map((member) => member.pubkey)
      .map((pubkey) => pubkey.trim().toLowerCase()),
  );
  const memberLabels = getChannelAgentLabels(members);
  const ids = new Set<string>();

  for (const agent of managedAgents) {
    if (
      agent.personaId &&
      memberPubkeys.has(agent.pubkey.trim().toLowerCase())
    ) {
      ids.add(agent.personaId);
    }
  }

  for (const persona of personas) {
    const label = normalizeChannelAgentLabel(persona.displayName);
    if (label !== null && memberLabels.has(label)) {
      ids.add(persona.id);
    }
  }

  return ids;
}
