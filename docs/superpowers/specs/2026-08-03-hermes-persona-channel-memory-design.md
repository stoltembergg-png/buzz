# Hermes Persona + Channel Local Memory Design

## Summary

Buzz will assign each Hermes runtime instance a deterministic local `HERMES_HOME` scoped to the active persona and channel. This gives the same persona in the same channel persistent local memory across sessions while isolating memory between different personas, channels, communities, and relays.

No memory content is published to the Buzz relay. Other ACP runtimes remain unchanged.

## Goals

- Persist Hermes-native memory for a persona/channel pair across agent restarts.
- Isolate memory between personas and between channels.
- Keep memory local to the current device.
- Reuse Hermes' existing memory, session, skills, and state layout instead of duplicating it in Buzz.
- Avoid exposing credentials or memory content in relay events, logs, UI payloads, or serialized runtime metadata.
- Preserve existing behavior for non-Hermes runtimes.

## Non-goals

- Synchronizing memory between devices.
- Publishing encrypted memory through Nostr or Buzz relay events.
- Sharing one global persona memory across all channels.
- Adding semantic search, memory editing UI, or memory inspection UI.
- Reimplementing Hermes memory storage or compression in Rust.
- Migrating existing Hermes global memories into scoped profiles.

## Scope identity

A memory scope is identified by the tuple:

```text
relay/community identity + channel ID + persona identity
```

The implementation must use stable machine identifiers, not display names. Display names may change and may contain unsafe path characters.

Preferred inputs:

- Relay or community: canonical relay URL and community identifier when available.
- Channel: stable channel UUID/coordinate.
- Persona: stable persona ID, agent definition ID, or public key-backed identifier already used by Buzz.

If a stable persona ID is unavailable at one entrypoint, the runtime configuration must derive a deterministic identity from the canonical persona snapshot rather than its mutable display label.

## Path layout

Buzz will resolve a root under its local application data directory:

```text
<Buzz app data>/hermes/profiles/v1/<scope-hash>/
```

`scope-hash` is a lowercase hexadecimal cryptographic digest over a versioned canonical scope string. Raw relay URLs, channel IDs, persona names, or user-controlled path fragments are never inserted directly into the filesystem path.

The versioned canonical input is:

```text
buzz-hermes-memory-v1\n<relay-scope>\n<channel-id>\n<persona-id>
```

A single flat hash directory is preferred over nested user-derived directories because it:

- eliminates traversal and reserved-name risks;
- avoids platform-specific path length issues;
- keeps identity derivation atomic;
- permits future scope-version migrations.

The scoped directory becomes the Hermes home and may contain Hermes-managed files such as:

```text
config.yaml
memories/
sessions/
state/
skills/
```

Buzz does not interpret or publish the contents of those paths.

## Configuration bootstrap

On first use, Buzz creates the scoped directory idempotently.

If the scoped `config.yaml` does not exist, Buzz copies the user's primary Hermes `config.yaml` into it using a safe create-if-absent operation. This preserves configured provider/model behavior without copying memories or session state.

Bootstrap rules:

1. Respect the currently resolved primary Hermes home (`HERMES_HOME` when set, otherwise Hermes' default home).
2. Copy only `config.yaml`.
3. Never copy `memories`, `sessions`, `state`, caches, OAuth stores, `.env`, or secret-store material.
4. Never overwrite an existing scoped `config.yaml`.
5. If no primary `config.yaml` exists, create only the scoped directory and allow Hermes defaults to apply.
6. Filesystem bootstrap failures are explicit runtime-start errors with the path redacted to an app-relative diagnostic where practical.

The copy is local filesystem initialization; configuration contents must not be included in logs or runtime discovery responses.

## Runtime integration

The feature applies only when the normalized runtime identity is one of:

```text
hermes
hermes-agent
hermes-acp
```

Before spawning Hermes, Buzz:

1. Resolves the stable memory scope.
2. Creates/bootstrap the scoped home.
3. Injects `HERMES_HOME=<scoped-home>` into the Hermes process environment.

The scoped `HERMES_HOME` must be set explicitly by the trusted Buzz host even if the parent process already has `HERMES_HOME`. For non-Hermes runtimes, Buzz must neither inject nor alter `HERMES_HOME`.

Existing Hermes-specific environment bridges for Buzz relay access and reasoning effort remain separate from memory-scope derivation.

## Data flow

```text
Persona selected + channel opened
            |
            v
Buzz derives stable scope tuple
            |
            v
Hash versioned canonical scope
            |
            v
Ensure local scoped Hermes home
            |
            v
Spawn Hermes ACP with HERMES_HOME
            |
            v
Hermes loads and updates native memory locally
```

When the same persona is started again in the same channel and relay/community, Buzz derives the same path. Changing any scope component produces a different path.

## Concurrency and lifecycle

Directory creation is idempotent and safe when multiple startup paths race.

Configuration bootstrap uses create-if-absent semantics. A process that loses the race must use the file created by the winner rather than overwrite it.

Buzz does not delete scoped memory when:

- an agent process exits;
- a channel is archived;
- a persona is removed from a channel.

Deletion and retention controls are deferred to a later feature because automatic deletion risks destroying user-owned memories.

## Security and privacy

- Memory never enters relay events as part of this feature.
- Raw scope values are hashed before filesystem use.
- Memory contents and copied configuration are excluded from logs.
- Runtime discovery may expose that scoped memory is enabled, but not its absolute path or contents.
- The feature does not broaden which processes receive Buzz credentials.
- Directory creation follows the application's existing local-data permissions. On Unix-like systems, newly created directories should not intentionally widen permissions beyond the process umask.
- Symbolic-link handling must be fail-closed for bootstrap writes: Buzz must not overwrite an existing target through a symlink while copying configuration.

## Error handling

Startup fails with a clear, non-secret error when:

- the app data directory cannot be resolved;
- the scoped directory cannot be created;
- an unsafe existing filesystem object blocks the scope path;
- configuration bootstrap cannot safely complete.

A missing primary Hermes configuration is not an error.

Memory read/write failures occurring later inside Hermes remain Hermes runtime errors and flow through the existing ACP error surface.

## Testing

### Unit tests

- Same relay, channel, and persona produce the same scope hash and path.
- Different channels produce different paths.
- Different personas produce different paths.
- Different relay/community identities produce different paths.
- Scope derivation is independent of display labels.
- Malicious or unusual identifiers cannot affect parent directories.
- Hermes aliases are recognized; unrelated runtimes are rejected.
- Non-Hermes processes do not receive a memory-scoped `HERMES_HOME`.

### Filesystem tests

- Scoped directory creation is idempotent.
- Primary `config.yaml` is copied only on first initialization.
- Existing scoped configuration is never overwritten.
- Missing primary configuration succeeds.
- Memories and sessions are not copied.
- Symlink or non-directory conflicts fail closed.

### Integration tests

- Agent spawn environment contains scoped `HERMES_HOME` for Hermes.
- Two channels for one persona receive distinct homes.
- Two personas in one channel receive distinct homes.
- Restarting the same persona/channel receives the original home.
- Existing reasoning-effort and Buzz credential bridges remain functional.

## Compatibility

The path format is versioned as `v1`. Future two-layer memory or synchronized memory features may introduce a new layout without changing existing local data.

Existing users begin with empty scoped memories. Their original global Hermes memory remains untouched in the primary Hermes home.

## Delivery

The implementation will be submitted as a focused Pull Request containing:

- scope identity and path derivation;
- safe scoped-home bootstrap;
- Hermes-only spawn integration;
- tests and documentation;
- no relay protocol changes and no UI memory editor.
