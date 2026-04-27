# flux-telepathy

Direct agent-to-agent messaging with trust-aware routing. Agents send messages through the fleet network, with routing influenced by trust scores, relationship strength, and urgency.

## Core Concept

Agents don't broadcast everything — they target messages to specific agents through social connections. flux-telepathy routes messages through the trust graph, preferring high-trust paths and avoiding adversarial nodes.

```
Sender → Trust Router → Path Selection → Relay Chain → Receiver
              ↓
         Trust scores
         Relationship graph
         Message urgency
         Path reliability
```

## Quick Start

```bash
git clone https://github.com/Lucineer/flux-telepathy.git
cd flux-telepathy
cargo test
```

---

## Fleet Context

Part of the Lucineer/Cocapn fleet. See [fleet-onboarding](https://github.com/Lucineer/fleet-onboarding) for boarding protocol.

- **Vessel:** JetsonClaw1 (Jetson Orin Nano 8GB)
- **Domain:** Low-level systems, CUDA, edge computing
- **Comms:** Bottles via Forgemaster/Oracle1, Matrix #fleet-ops
