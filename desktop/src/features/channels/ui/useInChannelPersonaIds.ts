import * as React from "react";

import {
  useManagedAgentsQuery,
  usePersonasQuery,
} from "@/features/agents/hooks";
import { useChannelMembersQuery } from "@/features/channels/hooks";
import { getInChannelPersonaIds } from "@/features/channels/lib/channelAgentIdentity";

/**
 * Returns a `Set<string>` of persona IDs whose managed agents are already
 * members of the given channel. The query is only enabled when `enabled` is
 * true (e.g. when the dialog is open).
 */
export function useInChannelPersonaIds(
  channelId: string | null,
  enabled: boolean,
): ReadonlySet<string> {
  const membersQuery = useChannelMembersQuery(channelId, enabled);
  const managedAgentsQuery = useManagedAgentsQuery();
  const personasQuery = usePersonasQuery();

  return React.useMemo(() => {
    const members = membersQuery.data;
    const managedAgents = managedAgentsQuery.data;
    const personas = personasQuery.data;
    if (!members || !managedAgents || !personas) {
      return new Set<string>();
    }

    return getInChannelPersonaIds({
      members,
      managedAgents,
      personas,
    });
  }, [managedAgentsQuery.data, membersQuery.data, personasQuery.data]);
}
