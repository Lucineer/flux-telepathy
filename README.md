# flux-telepathy 🧠

**Direct agent-to-agent messaging with trust-aware routing.** Agents send messages through the fleet network, with routing influenced by trust scores, relationship strength, and urgency.

```rust
use flux_telepathy::{Telepathy, MsgKind};

let mut tp = Telepathy::new();
tp.send(/* from: */ 1, /* to: */ 2, MsgKind::Report, "status: ok", 0.85);
tp.broadcast(/* from: */ 1, "network: stable", 0.9);
let messages = tp.receive(/* agent: */ 2);
```

## Why Telepathy?

Broadcast doesn't scale. Point-to-point messaging — routed through the trust graph — keeps noise low and signal high:

- **Emergency messages** always get through (priority 10, bypasses trust filter)
- **Low-trust agents** are silently routed around (configurable threshold)
- **Priority filtering** — a busy agent reads urgent messages first
- **No external dependencies** — pure Rust, no tokio, no networking, just data structures

## API

```rust
// Create a new telepathy router
let mut tp = Telepathy::new();

// Send directed message — returns message ID
let msg_id = tp.send(1, 2, MsgKind::Tell, "hello", 0.8);

// Broadcast (to=0 means everyone)
tp.broadcast(1, "network status: ok", 0.9);

// Emergency bypasses trust filtering
tp.emergency(3, "node 3 overheating");

// Receive unread messages for an agent
let inbox = tp.receive(2);

// Priority-aware receive
let urgent = tp.receive_priority(2, 8);  // only priority >= 8

// Trust filtering
let trusted = tp.filter_trust(0.7);

// Mark read
tp.mark_read(msg_id);

// Query
println!("Agent 2 has {} unread messages", tp.inbox_count(2));

// Configure
tp.set_trust_threshold(0.5);  // ignore messages from agents with trust < 0.5
```

### Message Types

| Kind | Priority | Purpose |
|------|----------|---------|
| `Tell` | 3 | Direct message, informational |
| `Ask` | 3 | Request for information |
| `Delegate` | 3 | Task assignment |
| `Broadcast` | 2 | Announcement to fleet |
| `Report` | 5 | Status update, telemetry |
| `Emergency` | 10 | Critical — always delivered |

## Cargo.toml

```toml
[dependencies]
flux-telepathy = { git = "https://github.com/Lucineer/flux-telepathy" }
```

## Fleet Context

Part of the Lucineer/Cocapn fleet. See [fleet-onboarding](https://github.com/Lucineer/fleet-onboarding) for boarding protocol. Sister crate to [flux-stigmergy](https://github.com/Lucineer/flux-stigmergy) (indirect env-based comms) — use in combination for resilient fleet coordination.
