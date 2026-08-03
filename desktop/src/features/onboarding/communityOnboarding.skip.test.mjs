import test from "node:test";
import assert from "node:assert/strict";

import { completeCommunityOnboardingAfterSkip } from "./communityOnboarding.tsx";

test("skip reconciles public starter channels before closing onboarding", async () => {
  const calls = [];
  const completed = [];
  const cleared = [];
  const queryClient = { id: "query-client" };
  let resolveInitialization;
  const initialization = new Promise((resolve) => {
    resolveInitialization = resolve;
  });

  const reconciliation = completeCommunityOnboardingAfterSkip({
    queryClient,
    pubkey: "guest-pubkey",
    relayUrl: "wss://relay.example",
    initializeStarterChannels: async (client, options) => {
      calls.push({ client, options });
      return initialization;
    },
    markComplete: (pubkey, relayUrl) => completed.push({ pubkey, relayUrl }),
    clear: () => cleared.push(true),
  });

  assert.deepEqual(completed, [
    { pubkey: "guest-pubkey", relayUrl: "wss://relay.example" },
  ]);
  assert.deepEqual(cleared, [true]);
  resolveInitialization({ ok: true });
  await reconciliation;
  assert.deepEqual(calls, [
    {
      client: queryClient,
      options: {
        focus: false,
        pubkey: "guest-pubkey",
        communityScope: "wss://relay.example",
      },
    },
  ]);
  assert.deepEqual(completed, [
    { pubkey: "guest-pubkey", relayUrl: "wss://relay.example" },
  ]);
  assert.deepEqual(cleared, [true]);
});

test("skip still exits onboarding when relay reconciliation is temporarily unavailable", async () => {
  const completed = [];
  const cleared = [];

  await completeCommunityOnboardingAfterSkip({
    queryClient: {},
    pubkey: "guest-pubkey",
    relayUrl: "wss://relay.example",
    initializeStarterChannels: async () => {
      throw new Error("relay unavailable");
    },
    markComplete: (pubkey, relayUrl) => completed.push({ pubkey, relayUrl }),
    clear: () => cleared.push(true),
  });

  assert.equal(completed.length, 1);
  assert.equal(cleared.length, 1);
});
