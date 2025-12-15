#!/bin/bash
# Deploy Alexandria Landing Page to mainnet

set -e

echo "Building and deploying alex_landing_page_frontend to mainnet..."
dfx deploy --network ic alex_landing_page_frontend

echo ""
echo "Deployed to: https://z6d57-uyaaa-aaaau-ac24a-cai.icp0.io"
