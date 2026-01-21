import { useState, useEffect } from 'react';
import './index.scss';

function App() {
  const words = [
    'VetKey Encryption',
    'Chain-Key ECDSA',
    'Threshold Signatures',
    'Provably Fair VRF',
    'Orthogonal Persistence',
    'ArWeave Mirroring',
    'Blackholed Canisters',
    'Deflationary Burns',
    'LP Locking Primitives',
    'DAO LLC Wrappers',
    'Revenue-Share NFTs',
    'Dual-Token Economics',
    'Cross-Chain Settlement',
    'Certified Variables',
    'HTTP Outcalls',
    'Reverse Gas Models',
    'WASM Smart Contracts',
    'Cycle Burn Analytics'
  ];

  const [displayText, setDisplayText] = useState('');
  const [wordIndex, setWordIndex] = useState(0);
  const [isDeleting, setIsDeleting] = useState(false);

  useEffect(() => {
    const currentWord = words[wordIndex];

    const timeout = setTimeout(() => {
      if (!isDeleting) {
        if (displayText.length < currentWord.length) {
          setDisplayText(currentWord.slice(0, displayText.length + 1));
        } else {
          setTimeout(() => setIsDeleting(true), 2000);
        }
      } else {
        if (displayText.length > 0) {
          setDisplayText(displayText.slice(0, -1));
        } else {
          setIsDeleting(false);
          setWordIndex((prev) => (prev + 1) % words.length);
        }
      }
    }, isDeleting ? 50 : 100);

    return () => clearTimeout(timeout);
  }, [displayText, isDeleting, wordIndex]);

  const products = [
    {
      name: 'lbry.app',
      tagline: 'The library and city center of Alexandria. Home of the $ALEX token—100% of revenue from all projects returns to stakers here.',
      description: 'LBRY.app is a decentralized archival platform for permanently preserving Web2 content as revenue-sharing NFTs. Built on ArWeave for permanent storage and ICP for compute, content creators mint NFTs that earn ongoing ALEX rewards whenever users interact with the network. The dual-token economy features LBRY as mintable operational currency and ALEX (hard-capped at 21M) for governance and staking revenue—with two-thirds of total supply distributed to NFT creators and content owners.',
      url: 'https://lbry.app',
      logo: '/logos/projects/alex.png',
      tags: ['DeFi', 'ContentFi', 'Infrastructure']
    },
    {
      name: 'CycleScan',
      tagline: 'The cycle burn leaderboard for ICP. Like CoinGecko, but for cycle consumption.',
      description: 'CycleScan tracks real-time cycle consumption across the Internet Computer, monitoring ~2,900 canisters from 100+ projects. Features include 7-day burn rate trends with sparkline visualizations, top-up detection with inferred burn calculations, and project-level aggregation. Data updates hourly via GitHub Actions with no backend canister required—a fully static architecture that fetches live data directly from raw GitHub URLs.',
      url: 'https://xknwi-uaaaa-aaaak-qu4oq-cai.icp0.io',
      logo: '/logos/projects/cyclescan.png',
      tags: ['Infrastructure', 'Analytics']
    },
    {
      name: 'IPG',
      tagline: 'The first fully decentralized payment gateway. Accept BTC and ICP with zero setup fees, no KYC, and sub-second payment detection.',
      description: 'Internet Payment Gateway is a first-of-its-kind merchant solution uniquely possible on ICP—leveraging HTTP outcalls for real-time browser-based payment polling and chain-key ckBTC for trustless Bitcoin settlement. No intermediaries, no monthly fees, no KYC. Just copy-paste a widget into any website and start accepting crypto. Regressive withdrawal fees (1.0% → 0.2%) reward volume, and all funds remain fully self-custodial until you withdraw.',
      url: 'https://62rey-3aaaa-aaaag-acjla-cai.icp0.io',
      logo: '/logos/projects/ipg.png',
      tags: ['Payments', 'Infrastructure', 'DeFi']
    },
    {
      name: 'OpenHouse',
      tagline: 'A decentralized casino where anyone can be the house, using ICP\'s unique randomness for skill-based and prediction market gaming.',
      description: 'OpenHouse is a provably fair casino where you can be the house. Users provide liquidity and earn from a transparent 1% house edge, with all randomness cryptographically verified via Internet Computer\'s VRF. Games include Crash, Plinko, Blackjack, and Dice—all open-source with publicly auditable odds. 100% of platform revenue flows to ALEX stakers.',
      url: 'https://openhouse.games',
      logo: '/logos/projects/openhouse.png',
      tags: ['Gaming', 'DeFi']
    },
    {
      name: 'DAOPad',
      tagline: 'Turning LLCs into Smart Contracts.',
      description: 'DAOPad turns LLCs into Smart Contracts, unifying company structure, legal agreements, and operations into a single autonomous system. Winner of DFINITY\'s WCHL2025 Hackathon (Real-World-Asset Track), it provides a white-glove framework for migrating to a legally recognized Wyoming DAO LLC—complete with business bank accounts, fiat off-ramps, and the ability to own property and sign contracts. Governance is powered by locked KongSwap LP tokens, with 1% of dealflow distributed to ALEX stakers.',
      url: 'https://daopad.org',
      logo: '/logos/projects/daopad_logo.png',
      tags: ['DAO', 'RWA', 'DeFi']
    },
    {
      name: 'KongLocker',
      tagline: 'The only way to permanently lock liquidity on KongSwap, with proof of retained LP ownership.',
      description: 'KongLocker is a blackholed liquidity locking service for KongSwap. Projects can permanently lock their LP tokens to demonstrate long-term commitment, with the canister tracking individual shares of locked liquidity. These locked LP positions serve as governance weight in DAOPad, granting voting power proportional to permanently committed value.',
      url: 'https://konglocker.com',
      logo: '/logos/projects/kong_locker.png',
      tags: ['DeFi', 'Infrastructure']
    },
    {
      name: 'Caffeine Launcher',
      tagline: 'A fair launch mechanism for Caffeine.AI projects, enabling results-based investments over time.',
      description: 'Caffeine Launcher is a token launchpad on the Internet Computer featuring dual-token distribution mechanics. Projects can fair-launch with a 1% fee that automatically flows to buying and burning LBRY tokens, creating deflationary pressure tied directly to platform activity. Established tokens can integrate at no cost, with all launched tokens eligible for DAOPad governance integration.',
      url: 'https://caffeinelauncher.com',
      logo: '/logos/projects/lbry_fun.svg',
      tags: ['DeFi', 'Launchpad']
    }
  ];

  const metrics = [
    { value: '50K+', label: 'Revshare NFTs Minted' },
    { value: '7', label: 'Production dApps' },
    { value: 'WCHL2025', label: 'Hackathon Winner' },
    { value: '100%', label: 'Fair Launched' }
  ];

  return (
    <main className="landing-page">
      <section className="hero">
        <h1 className="title">{displayText}<span className="cursor">|</span></h1>
        <p className="subtitle">Building ICP's Great City</p>
      </section>

      <section className="metrics">
        {metrics.map((item) => (
          <div key={item.label} className="metric">
            <span className="metric__value">{item.value}</span>
            <span className="metric__label">{item.label}</span>
          </div>
        ))}
      </section>

      <section className="products-section">
        <div className="products-table">
          {products.map((product) => (
            <a
              key={product.name}
              href={product.url}
              className="table-row"
              target="_blank"
              rel="noopener noreferrer"
            >
              <div className="col-project">
                <img src={product.logo} alt="" />
                <span className="project-name">{product.name}</span>
                <div className="tags">
                  {product.tags.map((tag) => (
                    <span key={tag} className="tag">{tag}</span>
                  ))}
                </div>
              </div>
              <div className="col-tagline">{product.tagline}</div>
              <div className="col-description">{product.description}</div>
            </a>
          ))}
        </div>
      </section>

      <footer className="footer">
        <a href="https://x.com/alexandria_lbry" target="_blank" rel="noopener noreferrer" title="Twitter">
          <svg viewBox="0 0 24 24" fill="currentColor"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>
        </a>
        <a href="https://github.com/AlexandriaDAO" target="_blank" rel="noopener noreferrer" title="Github">
          <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
        </a>
        <a href="https://lbry.app/info/whitepaper" target="_blank" rel="noopener noreferrer" title="Whitepaper">
          <svg viewBox="0 0 24 24" fill="currentColor"><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/></svg>
        </a>
        <a href="https://lbry.app/info/audit" target="_blank" rel="noopener noreferrer" title="Audit">
          <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z"/></svg>
        </a>
        <a href="https://kongswap.io/stats/ysy5f-2qaaa-aaaap-qkmmq-cai" target="_blank" rel="noopener noreferrer" title="KongSwap">
          <img src="/logos/external/kongswap.png" alt="" />
        </a>
        <a href="https://app.icpswap.com/swap?input=ryjl3-tyaaa-aaaaa-aaaba-cai&output=ysy5f-2qaaa-aaaap-qkmmq-cai" target="_blank" rel="noopener noreferrer" title="ICPSwap">
          <img src="/logos/external/icpswap.png" alt="" />
        </a>
        <a href="https://dexscreener.com/icp/kb4fz-oiaaa-aaaag-qnema-cai" target="_blank" rel="noopener noreferrer" title="DexScreener">
          <img src="/logos/external/dexscreener.png" alt="" />
        </a>
        <a href="https://icptokens.net/token/ysy5f-2qaaa-aaaap-qkmmq-cai" target="_blank" rel="noopener noreferrer" title="ICPTokens">
          <img src="/logos/external/icptokens.png" alt="" />
        </a>
      </footer>
    </main>
  );
}

export default App;
