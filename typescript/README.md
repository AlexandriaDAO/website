# Alexandria Landing Page - TypeScript/React Version

The Alexandria DAO landing page built with React 18, Vite, and SCSS. Deployed as a static asset canister on the Internet Computer.

## Tech Stack

- **React 18** - UI framework
- **Vite** - Build tool
- **SCSS** - Styling
- **ICP Asset Canister** - Hosting

## Quick Start

```bash
# Install dependencies
npm install

# Build for production
npm run build

# Deploy to mainnet
dfx deploy --network ic alex_landing_page_frontend
```

## Development

```bash
# Start dev server (local testing)
npm start
# Opens at http://localhost:3000
```

## Project Structure

```
typescript/
├── src/
│   ├── App.jsx         # Main component with all sections
│   ├── main.jsx        # React entry point
│   └── index.scss      # Global styles
├── dist/               # Built assets (deployed to IC)
├── public/             # Static assets
│   └── logos/          # Project and exchange logos
├── dfx.json            # ICP canister config
├── canister_ids.json   # Canister ID mapping
├── package.json        # NPM dependencies
└── vite.config.js      # Vite configuration
```

## Deployment

All deployments go directly to mainnet:

```bash
# Build and deploy
npm run build
dfx deploy --network ic alex_landing_page_frontend
```

## Live Site

- **URL**: https://z6d57-uyaaa-aaaau-ac24a-cai.icp0.io
- **Canister ID**: `z6d57-uyaaa-aaaau-ac24a-cai`
