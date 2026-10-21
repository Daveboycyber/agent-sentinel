# Guardian Contract

On-chain policy engine for AgentSentinel.

## What it does (MVP)

- Register agent accounts under protection
- Store simple policies (`max_single_transfer`, `max_daily_volume`, `is_frozen`)
- Freeze / unfreeze agents
- `is_transfer_allowed(agent, amount, receiver)` view method for the watcher to query

## Build & Test

```bash
cd contracts/guardian
cargo test
cargo near build   # or near-sdk build tools
```

## Next steps

1. Deploy to testnet
2. Add velocity tracking (daily volume)
3. Add a dedicated "watcher" role so the off-chain service can call `freeze_agent`
4. Integrate with function-call keys later for stronger guarantees
