# Alexandria Landing Page - Claude Deployment Guide

## 🎯 CRITICAL: Mainnet-Only Landing Page

**⚠️ IMPORTANT: There is no local testing environment. ALL changes deploy directly to mainnet.**

This is the main Alexandria landing page hosted on the Internet Computer. Every change you make goes directly to the production canister.

## 🚀 Quick Start

```bash
# Deploy to mainnet
dfx deploy --network ic alex_landing_page_frontend

# Or use specific canister ID
dfx deploy --network ic alex_landing_page_frontend --specified-id z6d57-uyaaa-aaaau-ac24a-cai
```

## 📦 Canister Information

| Component | Canister ID | URL |
|-----------|-------------|-----|
| **Landing Page Frontend** | `z6d57-uyaaa-aaaau-ac24a-cai` | https://z6d57-uyaaa-aaaau-ac24a-cai.icp0.io |

## 🏗️ Development Workflow

### Step 1: Make Frontend Changes
```bash
# Navigate to frontend directory
cd src/alex_landing_page_frontend

# Edit your files
vim src/index.html
vim src/index.css
vim src/index.js
# (or any other frontend assets)
```

### Step 2: Build Frontend (if needed)
```bash
# If using a build process (npm, webpack, etc)
npm run build

# Otherwise, assets are deployed directly from dist/ folder
```

### Step 3: Deploy to Mainnet (MANDATORY)
```bash
# From project root
dfx deploy --network ic alex_landing_page_frontend

# The canister will be deployed to: z6d57-uyaaa-aaaau-ac24a-cai
```

### Step 4: Verify Deployment
```bash
# Check the live site
open https://z6d57-uyaaa-aaaau-ac24a-cai.icp0.io

# Check canister status
dfx canister --network ic status alex_landing_page_frontend

# Check canister info
dfx canister --network ic info z6d57-uyaaa-aaaau-ac24a-cai
```

### Step 5: Commit Changes
```bash
git add .
git commit -m "feat: update Alexandria landing page"
git push
```

## 🛠️ Project Structure

```
alex_landing_page/
├── dfx.json                              # DFX configuration (frontend-only)
├── src/
│   └── alex_landing_page_frontend/       # Landing page assets
│       ├── dist/                         # Deployable assets
│       │   ├── index.html               # Main HTML
│       │   ├── index.css                # Styles
│       │   ├── index.js                 # Scripts
│       │   └── assets/                  # Images, fonts, etc.
│       ├── package.json                 # Build dependencies (if any)
│       └── tsconfig.json                # TypeScript config (if used)
└── CLAUDE.md                            # This file
```

## 📝 dfx.json Configuration

The project is configured as a frontend-only asset canister:

```json
{
  "canisters": {
    "alex_landing_page_frontend": {
      "source": ["src/alex_landing_page_frontend/dist"],
      "type": "assets",
      "workspace": "alex_landing_page_frontend"
    }
  }
}
```

## 🔐 Canister Management

### Check Canister Cycles
```bash
dfx canister --network ic status alex_landing_page_frontend
```

### Top Up Canister (if needed)
```bash
dfx canister --network ic deposit-cycles <cycles_amount> alex_landing_page_frontend
```

### View Canister Controllers
```bash
dfx canister --network ic info z6d57-uyaaa-aaaau-ac24a-cai
```

## 🚨 Important Notes

### NO Backend
This project is **frontend-only**. There is no backend canister. All logic should be:
- Client-side JavaScript
- Calls to external canisters (if needed)
- Static HTML/CSS/JS assets

### Mainnet Only
- **NEVER** use `dfx start` or local development
- **ALWAYS** use `--network ic` flag
- Every deployment is **LIVE** immediately
- Test thoroughly before deploying

### Asset Canister Behavior
- All files in `src/alex_landing_page_frontend/dist/` are uploaded
- Files are served via the IC's HTTP gateway
- File paths become URL paths (e.g., `dist/about.html` → `/about.html`)
- `index.html` is served at root `/`

## 🐛 Common Issues & Solutions

### Issue: Deployment fails with "Canister already exists"
**Solution:** The canister ID is hardcoded in `canister_ids.json`. Just deploy normally:
```bash
dfx deploy --network ic alex_landing_page_frontend
```

### Issue: Changes not showing up after deployment
**Solution:**
1. Clear browser cache (Cmd/Ctrl + Shift + R)
2. Wait 1-2 minutes for IC to propagate changes
3. Check if you deployed to the right network (`--network ic`)

### Issue: "Permission denied" during deployment
**Solution:** Ensure you're using the correct identity:
```bash
dfx identity whoami
# Should show the identity that controls the canister
```

### Issue: Files not found after deployment
**Solution:** Ensure files are in the `dist/` folder before deploying:
```bash
ls -la src/alex_landing_page_frontend/dist/
# Should show your HTML, CSS, JS files
```

## 📊 Monitoring

### Check Deployment Status
```bash
# View canister information
dfx canister --network ic info z6d57-uyaaa-aaaau-ac24a-cai

# Check cycles balance
dfx canister --network ic status alex_landing_page_frontend

# View canister on dashboard
open https://dashboard.internetcomputer.org/canister/z6d57-uyaaa-aaaau-ac24a-cai
```

### View Live Site
```bash
# Main URL
open https://z6d57-uyaaa-aaaau-ac24a-cai.icp0.io

# Alternative IC gateway URLs
open https://z6d57-uyaaa-aaaau-ac24a-cai.raw.icp0.io
open https://z6d57-uyaaa-aaaau-ac24a-cai.raw.ic0.app
```

## 📝 Deployment Checklist

Before each deployment:
- [ ] Test HTML/CSS/JS changes locally (if possible)
- [ ] Ensure all assets are in `dist/` folder
- [ ] Check for broken links
- [ ] Verify images/assets load correctly
- [ ] Test responsive design (mobile/desktop)
- [ ] Check browser console for errors

After deployment:
- [ ] Visit live site and verify changes
- [ ] Test all navigation links
- [ ] Check mobile responsiveness
- [ ] Verify page load speed
- [ ] Check canister cycles balance
- [ ] Test on multiple browsers if making significant changes

## 🔗 Resources

- **Live Site**: https://z6d57-uyaaa-aaaau-ac24a-cai.icp0.io
- **IC Dashboard**: https://dashboard.internetcomputer.org/canister/z6d57-uyaaa-aaaau-ac24a-cai
- **IC Asset Canister Docs**: https://internetcomputer.org/docs/current/developer-docs/web-apps/application-frontends/overview

## ⚡ Key Principles

1. **ALWAYS deploy to mainnet** - No local environment exists
2. **Frontend-only architecture** - No backend canister
3. **Static assets only** - HTML, CSS, JS, images, fonts
4. **Test before deploy** - Changes are immediately live
5. **Monitor cycles** - Ensure canister stays funded
6. **Keep it simple** - This is a landing page, not a web app

---

**Remember**: You're working directly on mainnet. Every deployment affects the live Alexandria landing page immediately. Keep deployments clean and test thoroughly!
