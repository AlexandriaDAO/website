# Alexandria Landing Page (TypeScript) - Claude Guide

## Overview

React/TypeScript implementation of the Alexandria DAO landing page. This is the **production version** currently deployed.

## Canister Info

| Canister ID | URL |
|-------------|-----|
| `z6d57-uyaaa-aaaau-ac24a-cai` | https://z6d57-uyaaa-aaaau-ac24a-cai.icp0.io |

## Commands

```bash
# Install dependencies
npm install

# Build
npm run build

# Deploy to mainnet (LIVE immediately)
dfx deploy --network ic alex_landing_page_frontend

# Check canister status
dfx canister --network ic status alex_landing_page_frontend
```

## Structure

```
├── src/
│   ├── App.jsx         # All components (Hero, Metrics, Products, Footer)
│   ├── main.jsx        # React entry
│   └── index.scss      # Styles (dark theme, responsive)
├── dist/               # Build output -> deployed to IC
├── public/logos/       # Project & exchange logos
├── dfx.json            # Canister config
└── canister_ids.json   # z6d57-uyaaa-aaaau-ac24a-cai
```

## Key Files

- **App.jsx** - Contains all 7 products, metrics, animated typewriter, footer links
- **index.scss** - Dark theme (#000), purple accent (#7c3aed), Space Grotesk + Roboto fonts
- **dist/** - What gets deployed (run `npm run build` first)

## Important

- **Mainnet only** - No local replica, all deploys are LIVE
- Always run `npm run build` before `dfx deploy`
- Clear browser cache after deploy if changes don't appear
