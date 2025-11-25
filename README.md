# Magnum: The Logos Adversarial Suite

Magnum is a stress-testing tool for trading bots, powered by **Rust** (CLI), **Python** (Math Core), and **Z3** (Symbolic Reasoning).

## Features
1. **The Pathologist:** Forensics analysis of liquidation events. Proves "Liquidity Voids" using Z3.
2. **The Crash Generator:** Local proxy server that injects synthetic Flash Crashes (Merton Model) into live market data.

## Quick Start (Docker)

No installation required. Just run:

```bash
# 1. Start the Crash Generator (Fake Exchange)
docker compose up -d crash-generator

# 2. Attack your bot
# Point your bot to http://localhost:8080
# Enable crash mode: GET /api/v3/klines?...&crash_mode=true
Manual Build
code
Bash
git submodule update --init --recursive
docker compose build
