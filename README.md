# flux-telepathy

> Agent-to-agent messaging with trust-weighted priority — the nervous system of the fleet.

## What It Is

Rust library for inter-agent communication in the FLUX ecosystem. Messages are routed based on trust scores — agents with higher trust get priority routing and faster delivery.

## Usage

```toml
[dependencies]
flux-telepathy = "0.1"
```

## Fleet Context

Part of the [FLUX agent simulation](https://github.com/Lucineer/flux-agent-sim) ecosystem. See also:
- [flux-confidence](https://github.com/Lucineer/flux-confidence) — belief propagation
- [flux-keeper](https://github.com/Lucineer/flux-keeper) — health monitoring
- [flux-energy](https://crates.io/crates/flux-energy) — ATP budgets

## License

MIT / Apache-2.0
