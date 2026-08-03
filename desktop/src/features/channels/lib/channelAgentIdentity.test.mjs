import test from "node:test";
import assert from "node:assert/strict";

import {
  getChannelAgentLabels,
  getInChannelPersonaIds,
  hasChannelAgentLabel,
  isDuplicateChannelAgentMember,
} from "./channelAgentIdentity.ts";

test("treats a remote bot with the same persona label as already in the channel", () => {
  const members = [
    {
      pubkey: "remote-fizz",
      displayName: "Fizz",
      isAgent: true,
      role: "bot",
    },
  ];

  assert.equal(hasChannelAgentLabel(members, " fizz "), true);
  assert.deepEqual([...getChannelAgentLabels(members)], ["fizz"]);
  assert.deepEqual(
    [
      ...getInChannelPersonaIds({
        members,
        managedAgents: [{ pubkey: "local-fizz", personaId: "fizz-persona" }],
        personas: [{ id: "fizz-persona", displayName: "Fizz" }],
      }),
    ],
    ["fizz-persona"],
  );
});

test("does not reserve a persona label used only by a human member", () => {
  assert.equal(
    hasChannelAgentLabel(
      [
        {
          pubkey: "human",
          displayName: "Fizz",
          isAgent: false,
          role: "member",
        },
      ],
      "Fizz",
    ),
    false,
  );
});

test("detects a duplicate channel agent by label but keeps the same pubkey idempotent", () => {
  const members = [
    {
      pubkey: "remote-fizz",
      displayName: "Fizz",
      isAgent: true,
      role: "bot",
    },
  ];

  assert.equal(
    isDuplicateChannelAgentMember(
      { pubkey: "local-fizz", name: "Fizz" },
      members,
    ),
    true,
  );
  assert.equal(
    isDuplicateChannelAgentMember(
      { pubkey: "REMOTE-FIZZ", name: "Fizz" },
      members,
    ),
    false,
  );
  assert.equal(
    isDuplicateChannelAgentMember({ pubkey: "human-fizz", name: "Fizz" }, [
      { ...members[0], pubkey: "human-fizz", isAgent: false, role: "member" },
    ]),
    false,
  );
});
test("normalizes duplicate labels case-insensitively and ignores non-agents", () => {
  const members = [
    { pubkey: "a", displayName: " FIZZ ", isAgent: true, role: "member" },
    { pubkey: "b", displayName: "Fizz", isAgent: true, role: "bot" },
    { pubkey: "c", displayName: "Fizz", isAgent: false, role: "member" },
  ];
  assert.deepEqual([...getChannelAgentLabels(members)], ["fizz"]);
});
