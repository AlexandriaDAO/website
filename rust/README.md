# Alexandria Landing Page - Rust/Leptos Version

The Alexandria DAO landing page built with Leptos and compiled to WebAssembly. An experimental pure-Rust frontend alternative to the TypeScript version.

## Tech Stack

- **Leptos 0.6** - Rust frontend framework
- **WebAssembly** - Compiled target
- **Trunk** - Build tool
- **ICP Asset Canister** - Hosting

## Quick Start

```bash
# Install Trunk (if needed)
cargo install trunk

# Build for production
trunk build --release

# Deploy to mainnet
dfx deploy --network ic alex_landing_page_frontend
```

## Development

```bash
# Serve locally with hot reload
trunk serve --port 8080
# Opens at http://127.0.0.1:8080
```

## Project Structure

```
rust/
├── src/
│   └── lib.rs          # All Leptos components
├── style.css           # CSS styling
├── index.html          # HTML entry + Trunk directives
├── dist/               # Built WASM (deployed to IC)
├── public/             # Static assets
│   └── logos/          # Project and exchange logos
├── Cargo.toml          # Rust dependencies
├── Trunk.toml          # Trunk configuration
├── dfx.json            # ICP canister config
└── canister_ids.json   # Canister ID mapping
```

## Features

- Animated typewriter effect cycling through ICP technology keywords
- 7 product showcases with descriptions and tags
- Metrics section (NFTs minted, dApps, hackathon winner)
- Footer with social links and exchange integrations
- Responsive dark theme design

## Deployment

All deployments go directly to mainnet:

```bash
# Build and deploy
trunk build --release
dfx deploy --network ic alex_landing_page_frontend
```

## Live Site

- **URL**: https://z6d57-uyaaa-aaaau-ac24a-cai.icp0.io
- **Canister ID**: `z6d57-uyaaa-aaaau-ac24a-cai`

## Why Rust?

This is an experimental frontend to demonstrate that production web apps can be built entirely in Rust, compiled to WASM, and deployed to the Internet Computer - no JavaScript required.
