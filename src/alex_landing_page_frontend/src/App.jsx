import './index.scss';

function App() {
  const products = [
    {
      name: 'lbry.app',
      description: 'LBRY.app is a decentralized archival platform for permanently preserving Web2 content as revenue-sharing NFTs. Built on ArWeave for permanent storage and ICP for compute, content creators mint NFTs that earn ongoing ALEX rewards whenever users interact with the network. The dual-token economy features LBRY as mintable operational currency and ALEX (hard-capped at 21M) for governance and staking revenue—with two-thirds of total supply distributed to NFT creators and content owners.',
      url: 'https://lbry.app',
      logo: '/logos/alex.png',
      tags: ['DeFi', 'ContentFi', 'Infrastructure']
    },
    {
      name: 'OpenHouse',
      description: 'OpenHouse is a provably fair casino where you can be the house. Users provide liquidity and earn from a transparent 1% house edge, with all randomness cryptographically verified via Internet Computer\'s VRF. Games include Crash, Plinko, Blackjack, and Dice—all open-source with publicly auditable odds. 100% of platform revenue flows to ALEX stakers.',
      url: 'https://openhouse.games',
      logo: '/logos/openhouse.png',
      tags: ['Gaming', 'DeFi']
    },
    {
      name: 'DAOPad',
      description: 'DAOPad turns LLCs into Smart Contracts, unifying company structure, legal agreements, and operations into a single autonomous system. Winner of DFINITY\'s WCHL2025 Hackathon (Real-World-Asset Track), it provides a white-glove framework for migrating to a legally recognized Wyoming DAO LLC—complete with business bank accounts, fiat off-ramps, and the ability to own property and sign contracts. Governance is powered by locked KongSwap LP tokens, with 1% of dealflow distributed to ALEX stakers.',
      url: 'https://daopad.org',
      logo: '/logos/third_party/daopad_logo.png',
      tags: ['DAO', 'RWA', 'DeFi']
    },
    {
      name: 'KongLocker',
      description: 'KongLocker is a blackholed liquidity locking service for KongSwap. Projects can permanently lock their LP tokens to demonstrate long-term commitment, with the canister tracking individual shares of locked liquidity. These locked LP positions serve as governance weight in DAOPad, granting voting power proportional to permanently committed value.',
      url: 'https://konglocker.com',
      logo: '/logos/third_party/kong_locker.png',
      tags: ['DeFi', 'Infrastructure']
    },
    {
      name: 'Caffeine Launcher',
      description: 'Caffeine Launcher is a token launchpad on the Internet Computer featuring dual-token distribution mechanics. Projects can fair-launch with a 1% fee that automatically flows to buying and burning LBRY tokens, creating deflationary pressure tied directly to platform activity. Established tokens can integrate at no cost, with all launched tokens eligible for DAOPad governance integration.',
      url: 'https://caffeinelauncher.com',
      logo: '/logos/third_party/lbry_fun.svg',
      tags: ['DeFi', 'Launchpad']
    }
  ];

  return (
    <>
      {/* Sticky Navigation Header */}
      <nav className="nav-header">
        <div className="nav-container">
          <a href="/" className="nav-logo">AlexandriaDAO</a>
          <ul className="nav-links">
            <li><a href="/">Home</a></li>
            <li><a href="#about">About</a></li>
            <li><a href="#products">Products</a></li>
            <li><a href="https://lbry.app" target="_blank" rel="noopener noreferrer">Docs</a></li>
          </ul>
        </div>
      </nav>

      <main className="landing-page">
        {/* Hero Section */}
        <section className="hero">
          <div className="hero-content">
            <h1 className="title">AlexandriaDAO</h1>
            <p className="subtitle">A Studio for ICP Projects</p>
            <p className="tagline">
              Building the decentralized future on the Internet Computer, powered by $ALEX
            </p>
          </div>
        </section>

        {/* Products Section */}
        <section className="section" id="products">
          <h2 className="section-title">Products</h2>
          <div className="products-grid">
            {products.map((product) => (
              <a
                key={product.name}
                href={product.url}
                className="product-card"
                target="_blank"
                rel="noopener noreferrer"
              >
                <div className="product-icon">
                  <img src={product.logo} alt={`${product.name} icon`} />
                </div>
                <div className="product-content">
                  <div className="product-header">
                    <h3 className="product-name">{product.name}</h3>
                  </div>
                  <div className="product-tags">
                    {product.tags.map((tag) => (
                      <span key={tag} className="product-tag">
                        {tag}
                      </span>
                    ))}
                  </div>
                  <p className="product-description">{product.description}</p>
                </div>
              </a>
            ))}
          </div>
        </section>

        {/* Footer */}
        <footer className="footer">
          <div className="footer-content">
            <div className="footer-section">
              <h3>Home</h3>
            </div>
            <div className="footer-section">
              <h3>About</h3>
            </div>
            <div className="footer-section">
              <h3>Products</h3>
            </div>
          </div>
          <div className="footer-bottom">
            <p>&copy; 2025 AlexandriaDAO. All rights reserved.</p>
          </div>
        </footer>
      </main>
    </>
  );
}

export default App;
