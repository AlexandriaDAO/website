import './index.scss';

function App() {
  const products = [
    {
      name: 'lbry.app',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.',
      url: 'https://lbry.app',
      logo: '/logos/alex.png',
      tags: ['Library', 'ContentFi']
    },
    {
      name: 'DAOPad',
      description: 'Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit.',
      url: 'https://daopad.org',
      logo: '/logos/third_party/daopad_logo.png',
      tags: ['DeFi', 'Tooling']
    },
    {
      name: 'KongLocker',
      description: 'Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae.',
      url: 'https://konglocker.com',
      logo: '/logos/third_party/kong_locker.png',
      tags: ['Infrastructure', 'Security']
    },
    {
      name: 'Caffeine Launcher',
      description: 'Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt porro quisquam est.',
      url: 'https://caffeinelauncher.com',
      logo: '/logos/third_party/lbry_fun.svg',
      tags: ['DeFi', 'Tooling']
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
