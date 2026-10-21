# AgentSentinel

**Real-time scam & anomaly detector for agent-to-agent money flows on NEAR**

Built for the [NEAR Hackathon 2026](https://nearhackathon.com) — Track 06: AI Security & Defense.

> Trust, but verify. Give AI money without giving it the keys to everything.

## Overview

AgentSentinel watches transfers between AI agents in near real-time, scores them for scam risk using policy rules, and can automatically freeze or throttle suspicious flows before funds leave.

### Core Features (MVP)
- Registry of protected agent accounts
- On-chain Guardian policy contract (spending limits, velocity, new counterparties, etc.)
- Off-chain watcher that monitors NEAR transactions
- Ability to freeze / pause outgoing transfers from flagged agents
- Simple dashboard for live feed + risk scores + freeze status

## Project Structure

```
agent-sentinel/
├── contracts/
│   └── guardian/          # Rust NEAR smart contract (policy engine + freeze)
├── watcher/               # TypeScript real-time transaction watcher
├── frontend/              # Dashboard (Next.js) — coming later
├── scripts/               # Agent simulators & demo helpers
└── README.md
```

## Tech Stack
- **Contract**: Rust + near-sdk
- **Watcher**: TypeScript
- **Frontend**: Next.js (planned)
- **Network**: NEAR testnet (for development)

## Status

🚧 Early development — Day 1 setup in progress.

## License
MIT
