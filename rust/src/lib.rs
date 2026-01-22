//! Alexandria Landing Page - Pure Rust Canvas Rendering

#![cfg(target_arch = "wasm32")]

use eframe::egui::{self, Color32, FontFamily, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};

#[derive(Clone, Copy)]
struct ColorPalette {
    bg_primary: Color32,
    text_primary: Color32,
    text_secondary: Color32,
    text_muted: Color32,
    border: Color32,
    tag_bg: Color32,
    tag_text: Color32,
    // Rust-themed accent colors
    accent_rust: Color32,    // Primary rust orange
    accent_copper: Color32,  // Warm copper
    accent_ember: Color32,   // Hot ember/fire
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            bg_primary: Color32::from_rgb(0, 0, 0),
            text_primary: Color32::from_rgb(255, 255, 255),
            text_secondary: Color32::from_rgba_unmultiplied(255, 255, 255, 178),
            text_muted: Color32::from_rgba_unmultiplied(255, 255, 255, 100),
            border: Color32::from_rgba_unmultiplied(255, 255, 255, 40),
            tag_bg: Color32::from_rgba_unmultiplied(255, 255, 255, 20),
            tag_text: Color32::from_rgba_unmultiplied(255, 255, 255, 200),
            // Rust logo palette - forged metal vibes
            accent_rust: Color32::from_rgb(247, 76, 0),      // #F74C00 - Rust orange
            accent_copper: Color32::from_rgb(183, 65, 14),   // #B7410E - Dark rust/copper
            accent_ember: Color32::from_rgb(255, 140, 50),   // #FF8C32 - Hot ember glow
        }
    }
}

/// Interpolate between two colors
fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    Color32::from_rgba_unmultiplied(
        (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
        (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
        (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
        (a.a() as f32 + (b.a() as f32 - a.a() as f32) * t) as u8,
    )
}

/// Tracks hover animation state for smooth transitions
#[derive(Default)]
struct HoverState {
    /// Maps product index to hover intensity (0.0 = not hovered, 1.0 = fully hovered)
    product_hover: std::collections::HashMap<usize, f32>,
    /// Scanline position for each hovered product (0.0 to 1.0)
    scanline_pos: std::collections::HashMap<usize, f32>,
    /// Metric hover intensities
    metric_hover: [f32; 4],
}

impl HoverState {
    /// Update hover intensity with smooth animation
    fn update_hover(&mut self, idx: usize, is_hovered: bool, dt: f32) -> f32 {
        let intensity = self.product_hover.entry(idx).or_insert(0.0);
        let target = if is_hovered { 1.0 } else { 0.0 };
        let speed = 8.0; // Animation speed
        *intensity += (target - *intensity) * speed * dt;
        *intensity = intensity.clamp(0.0, 1.0);
        *intensity
    }

    /// Update scanline animation (sweeps across when hovered)
    fn update_scanline(&mut self, idx: usize, is_hovered: bool, dt: f32) -> f32 {
        let pos = self.scanline_pos.entry(idx).or_insert(0.0);
        if is_hovered {
            *pos += dt * 0.8; // Scanline speed
            if *pos > 1.3 {
                *pos = -0.3; // Reset with buffer for smooth loop
            }
        } else {
            *pos = -0.3; // Reset when not hovered
        }
        *pos
    }

    /// Update metric hover
    fn update_metric(&mut self, idx: usize, is_hovered: bool, dt: f32) -> f32 {
        if idx >= 4 { return 0.0; }
        let target = if is_hovered { 1.0 } else { 0.0 };
        let speed = 10.0;
        self.metric_hover[idx] += (target - self.metric_hover[idx]) * speed * dt;
        self.metric_hover[idx].clamp(0.0, 1.0)
    }
}

struct Typography {
    title_size: f32,
    subtitle_size: f32,
    metric_value_size: f32,
    metric_label_size: f32,
    product_name_size: f32,
    tagline_size: f32,
    description_size: f32,
    tag_size: f32,
}

impl Typography {
    fn for_width(width: f32) -> Self {
        if width < 480.0 {
            Self { title_size: 28.0, subtitle_size: 14.0, metric_value_size: 18.0, metric_label_size: 9.0, product_name_size: 15.0, tagline_size: 12.0, description_size: 11.0, tag_size: 8.0 }
        } else if width < 768.0 {
            Self { title_size: 36.0, subtitle_size: 16.0, metric_value_size: 22.0, metric_label_size: 10.0, product_name_size: 16.0, tagline_size: 13.0, description_size: 12.0, tag_size: 9.0 }
        } else {
            Self { title_size: 52.0, subtitle_size: 20.0, metric_value_size: 26.0, metric_label_size: 11.0, product_name_size: 18.0, tagline_size: 14.0, description_size: 13.0, tag_size: 9.0 }
        }
    }
}

#[derive(Clone)]
struct Product {
    name: &'static str,
    tagline: &'static str,
    description: &'static str,
    url: &'static str,
    tags: &'static [&'static str],
    image: &'static str,
}

#[derive(Clone)]
struct Metric { value: &'static str, label: &'static str }

#[derive(Clone)]
struct FooterLink { url: &'static str, icon: FooterIcon, title: &'static str }

#[derive(Clone, Copy)]
enum FooterIcon {
    Twitter,
    Github,
    Whitepaper,
    Audit,
    // Image-based icons (external exchanges/services)
    KongSwap,
    IcpSwap,
    DexScreener,
    IcpTokens,
}

const SUBTITLE_TEXT: &str = "Building ICP's Great City";

const TYPEWRITER_WORDS: &[&str] = &[
    "VetKey Encryption",
    "Chain-Key ECDSA",
    "Threshold Signatures",
    "Provably Fair VRF",
    "Orthogonal Persistence",
    "ArWeave Mirroring",
    "Blackholed Canisters",
    "Deflationary Burns",
    "LP Locking Primitives",
    "DAO LLC Wrappers",
    "Revenue-Share NFTs",
    "Dual-Token Economics",
    "Cross-Chain Settlement",
    "Certified Variables",
    "HTTP Outcalls",
    "Reverse Gas Models",
    "WASM Smart Contracts",
    "Cycle Burn Analytics",
];

struct TypewriterState {
    word_index: usize,
    char_index: usize,
    is_deleting: bool,
    last_update: f64,
    pause_until: f64,
}

impl Default for TypewriterState {
    fn default() -> Self {
        Self {
            word_index: 0,
            char_index: 0,
            is_deleting: false,
            last_update: 0.0,
            pause_until: 0.0,
        }
    }
}

impl TypewriterState {
    fn update(&mut self, time: f64) -> &str {
        let current_word = TYPEWRITER_WORDS[self.word_index];

        // If we're pausing, return current state
        if time < self.pause_until {
            return &current_word[..self.char_index];
        }

        let type_speed = 0.1; // 100ms per character
        let delete_speed = 0.05; // 50ms per character
        let speed = if self.is_deleting { delete_speed } else { type_speed };

        if time - self.last_update >= speed {
            self.last_update = time;

            if !self.is_deleting {
                if self.char_index < current_word.len() {
                    // Type next character (handle UTF-8 properly)
                    self.char_index = current_word
                        .char_indices()
                        .nth(current_word[..self.char_index].chars().count() + 1)
                        .map(|(i, _)| i)
                        .unwrap_or(current_word.len());
                } else {
                    // Word complete, pause then start deleting
                    self.is_deleting = true;
                    self.pause_until = time + 2.0; // 2 second pause
                }
            } else {
                if self.char_index > 0 {
                    // Delete previous character (handle UTF-8 properly)
                    let chars_count = current_word[..self.char_index].chars().count();
                    if chars_count > 0 {
                        self.char_index = current_word
                            .char_indices()
                            .nth(chars_count - 1)
                            .map(|(i, _)| i)
                            .unwrap_or(0);
                    } else {
                        self.char_index = 0;
                    }
                } else {
                    // Word deleted, move to next word
                    self.is_deleting = false;
                    self.word_index = (self.word_index + 1) % TYPEWRITER_WORDS.len();
                }
            }
        }

        &current_word[..self.char_index]
    }

    fn cursor_visible(&self, time: f64) -> bool {
        // Blink cursor every 0.5 seconds
        ((time * 2.0) as i32) % 2 == 0
    }
}

const PRODUCTS: &[Product] = &[
    Product {
        name: "lbry.app",
        tagline: "The library and city center of Alexandria. Home of the $ALEX token—100% of revenue from all projects returns to stakers here.",
        description: "LBRY.app is a decentralized archival platform for permanently preserving Web2 content as revenue-sharing NFTs. Built on ArWeave for permanent storage and ICP for compute, content creators mint NFTs that earn ongoing ALEX rewards whenever users interact with the network. The dual-token economy features LBRY as mintable operational currency and ALEX (hard-capped at 21M) for governance and staking revenue—with two-thirds of total supply distributed to NFT creators and content owners.",
        url: "https://lbry.app",
        tags: &["DeFi", "ContentFi", "Infrastructure"],
        image: "/logos/projects/alex.png",
    },
    Product {
        name: "CycleScan",
        tagline: "The cycle burn leaderboard for ICP. Like CoinGecko, but for cycle consumption.",
        description: "CycleScan tracks real-time cycle consumption across the Internet Computer, monitoring ~2,900 canisters from 100+ projects. Features include 7-day burn rate trends with sparkline visualizations, top-up detection with inferred burn calculations, and project-level aggregation. Data updates hourly via GitHub Actions with no backend canister required—a fully static architecture that fetches live data directly from raw GitHub URLs.",
        url: "https://xknwi-uaaaa-aaaak-qu4oq-cai.icp0.io",
        tags: &["Infrastructure", "Analytics"],
        image: "/logos/projects/cyclescan.png",
    },
    Product {
        name: "IPG",
        tagline: "The first fully decentralized payment gateway. Accept BTC and ICP with zero setup fees, no KYC, and sub-second payment detection.",
        description: "Internet Payment Gateway is a first-of-its-kind merchant solution uniquely possible on ICP—leveraging HTTP outcalls for real-time browser-based payment polling and chain-key ckBTC for trustless Bitcoin settlement. No intermediaries, no monthly fees, no KYC. Just copy-paste a widget into any website and start accepting crypto. Regressive withdrawal fees (1.0% → 0.2%) reward volume, and all funds remain fully self-custodial until you withdraw.",
        url: "https://62rey-3aaaa-aaaag-acjla-cai.icp0.io",
        tags: &["Payments", "Infrastructure", "DeFi"],
        image: "/logos/projects/ipg.png",
    },
    Product {
        name: "OpenHouse",
        tagline: "A decentralized casino where anyone can be the house, using ICP's unique randomness for skill-based and prediction market gaming.",
        description: "OpenHouse is a provably fair casino where you can be the house. Users provide liquidity and earn from a transparent 1% house edge, with all randomness cryptographically verified via Internet Computer's VRF. Games include Crash, Plinko, Blackjack, and Dice—all open-source with publicly auditable odds. 100% of platform revenue flows to ALEX stakers.",
        url: "https://openhouse.games",
        tags: &["Gaming", "DeFi"],
        image: "/logos/projects/openhouse.png",
    },
    Product {
        name: "DAOPad",
        tagline: "Turning LLCs into Smart Contracts.",
        description: "DAOPad turns LLCs into Smart Contracts, unifying company structure, legal agreements, and operations into a single autonomous system. Winner of DFINITY's WCHL2025 Hackathon (Real-World-Asset Track), it provides a white-glove framework for migrating to a legally recognized Wyoming DAO LLC—complete with business bank accounts, fiat off-ramps, and the ability to own property and sign contracts. Governance is powered by locked KongSwap LP tokens, with 1% of dealflow distributed to ALEX stakers.",
        url: "https://daopad.org",
        tags: &["DAO", "RWA", "DeFi"],
        image: "/logos/projects/daopad_logo.png",
    },
    Product {
        name: "KongLocker",
        tagline: "The only way to permanently lock liquidity on KongSwap, with proof of retained LP ownership.",
        description: "KongLocker is a blackholed liquidity locking service for KongSwap. Projects can permanently lock their LP tokens to demonstrate long-term commitment, with the canister tracking individual shares of locked liquidity. These locked LP positions serve as governance weight in DAOPad, granting voting power proportional to permanently committed value.",
        url: "https://konglocker.com",
        tags: &["DeFi", "Infrastructure"],
        image: "/logos/projects/kong_locker.png",
    },
    Product {
        name: "Caffeine Launcher",
        tagline: "A fair launch mechanism for Caffeine.AI projects, enabling results-based investments over time.",
        description: "Caffeine Launcher is a token launchpad on the Internet Computer featuring dual-token distribution mechanics. Projects can fair-launch with a 1% fee that automatically flows to buying and burning LBRY tokens, creating deflationary pressure tied directly to platform activity. Established tokens can integrate at no cost, with all launched tokens eligible for DAOPad governance integration.",
        url: "https://caffeinelauncher.com",
        tags: &["DeFi", "Launchpad"],
        image: "/logos/projects/lbry_fun.svg",
    },
];

const METRICS: &[Metric] = &[
    Metric { value: "50K+", label: "NFTs Minted" },
    Metric { value: "7", label: "dApps" },
    Metric { value: "WCHL2025", label: "Winner" },
    Metric { value: "100%", label: "Fair Launch" },
];

const FOOTER_LINKS: &[FooterLink] = &[
    FooterLink { url: "https://x.com/alexandria_lbry", icon: FooterIcon::Twitter, title: "Twitter" },
    FooterLink { url: "https://github.com/AlexandriaDAO", icon: FooterIcon::Github, title: "Github" },
    FooterLink { url: "https://lbry.app/info/whitepaper", icon: FooterIcon::Whitepaper, title: "Whitepaper" },
    FooterLink { url: "https://lbry.app/info/audit", icon: FooterIcon::Audit, title: "Audit" },
    FooterLink { url: "https://kongswap.io/stats/ysy5f-2qaaa-aaaap-qkmmq-cai", icon: FooterIcon::KongSwap, title: "KongSwap" },
    FooterLink { url: "https://app.icpswap.com/swap?input=ryjl3-tyaaa-aaaaa-aaaba-cai&output=ysy5f-2qaaa-aaaap-qkmmq-cai", icon: FooterIcon::IcpSwap, title: "ICPSwap" },
    FooterLink { url: "https://dexscreener.com/icp/kb4fz-oiaaa-aaaag-qnema-cai", icon: FooterIcon::DexScreener, title: "DexScreener" },
    FooterLink { url: "https://icptokens.net/token/ysy5f-2qaaa-aaaap-qkmmq-cai", icon: FooterIcon::IcpTokens, title: "ICPTokens" },
];

pub struct AlexandriaApp {
    colors: ColorPalette,
    base_url: String,
    typewriter: TypewriterState,
    start_time: f64,
    last_frame_time: f64,
    hover: HoverState,
}

impl Default for AlexandriaApp {
    fn default() -> Self {
        // Get the base URL from the current window location
        let base_url = web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .unwrap_or_default();
        let start_time = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now() / 1000.0)
            .unwrap_or(0.0);
        Self {
            colors: ColorPalette::default(),
            base_url,
            typewriter: TypewriterState::default(),
            start_time,
            last_frame_time: start_time,
            hover: HoverState::default(),
        }
    }
}

impl AlexandriaApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Install image loaders for loading PNGs from URLs
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let app = Self::default();
        web_sys::console::log_1(&format!("[RUST] Base URL for images: {}", app.base_url).into());
        web_sys::console::log_1(&format!("[RUST] Example image URL: {}{}", app.base_url, "/logos/projects/alex.png").into());
        app
    }

    fn centered_label(&self, ui: &mut egui::Ui, text: &str, font_size: f32, color: Color32) {
        ui.label(egui::RichText::new(text).size(font_size).color(color));
    }

    /// Draw a cyberpunk glowing border around a rect
    fn draw_glow_border(&self, painter: &egui::Painter, rect: Rect, intensity: f32, time: f64) {
        if intensity < 0.01 { return; }

        let colors = self.colors;
        // Animate gradient position for "energy flow" effect
        let phase = (time * 2.0) as f32;

        // Multiple glow layers for depth
        for (i, (alpha_mult, expand)) in [(0.15, 8.0), (0.3, 4.0), (0.6, 2.0)].iter().enumerate() {
            let glow_rect = rect.expand(*expand * intensity);

            // Gradient cycles between rust → copper → ember (forged metal effect)
            let gradient_t = ((phase + i as f32 * 0.3).sin() * 0.5 + 0.5) * intensity;
            let glow_color = if gradient_t < 0.5 {
                lerp_color(colors.accent_copper, colors.accent_rust, gradient_t * 2.0)
            } else {
                lerp_color(colors.accent_rust, colors.accent_ember, (gradient_t - 0.5) * 2.0)
            };

            let alpha = (255.0 * alpha_mult * intensity) as u8;
            let color = Color32::from_rgba_unmultiplied(glow_color.r(), glow_color.g(), glow_color.b(), alpha);
            painter.rect_stroke(glow_rect, Rounding::same(4.0), Stroke::new(1.5, color));
        }
    }

    /// Draw an animated scanline sweep effect
    fn draw_scanline(&self, painter: &egui::Painter, rect: Rect, scanline_pos: f32, intensity: f32) {
        if intensity < 0.01 || scanline_pos < 0.0 || scanline_pos > 1.0 { return; }

        let colors = self.colors;
        let y = rect.top() + rect.height() * scanline_pos;
        let scan_height = 3.0;

        // Scanline gradient: transparent → cyan → transparent
        let _scan_rect = Rect::from_min_max(
            Pos2::new(rect.left(), y - scan_height),
            Pos2::new(rect.right(), y + scan_height),
        );

        // Draw horizontal gradient scanline
        let alpha = (180.0 * intensity) as u8;
        let scan_color = Color32::from_rgba_unmultiplied(
            colors.accent_ember.r(), colors.accent_ember.g(), colors.accent_ember.b(), alpha
        );

        // Main scanline
        painter.line_segment(
            [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
            Stroke::new(2.0 * intensity, scan_color),
        );

        // Glow above and below
        let glow_alpha = (60.0 * intensity) as u8;
        let glow_color = Color32::from_rgba_unmultiplied(
            colors.accent_ember.r(), colors.accent_ember.g(), colors.accent_ember.b(), glow_alpha
        );
        for offset in [2.0, 4.0, 6.0] {
            painter.line_segment(
                [Pos2::new(rect.left(), y - offset), Pos2::new(rect.right(), y - offset)],
                Stroke::new(1.0, glow_color),
            );
            painter.line_segment(
                [Pos2::new(rect.left(), y + offset), Pos2::new(rect.right(), y + offset)],
                Stroke::new(1.0, glow_color),
            );
        }
    }

    /// Draw a subtle background glow on hover
    fn draw_hover_bg(&self, painter: &egui::Painter, rect: Rect, intensity: f32) {
        if intensity < 0.01 { return; }

        let colors = self.colors;
        let alpha = (15.0 * intensity) as u8;
        let bg_color = Color32::from_rgba_unmultiplied(
            colors.accent_copper.r(), colors.accent_copper.g(), colors.accent_copper.b(), alpha
        );
        painter.rect_filled(rect, Rounding::same(4.0), bg_color);
    }

    /// Draw glowing tag on hover
    fn draw_tag_glow(&self, painter: &egui::Painter, rect: Rect, intensity: f32, time: f64) {
        if intensity < 0.01 { return; }

        let colors = self.colors;
        let phase = (time * 3.0) as f32;
        let pulse = (phase.sin() * 0.3 + 0.7) * intensity;

        // Outer glow
        let glow_rect = rect.expand(2.0 * pulse);
        let alpha = (80.0 * pulse) as u8;
        let glow_color = Color32::from_rgba_unmultiplied(
            colors.accent_ember.r(), colors.accent_ember.g(), colors.accent_ember.b(), alpha
        );
        painter.rect_stroke(glow_rect, Rounding::same(5.0), Stroke::new(1.0, glow_color));
    }

    fn render_footer_icon(&self, painter: &egui::Painter, icon: FooterIcon, center: Pos2, size: f32, color: Color32) {
        match icon {
            FooterIcon::Twitter => {
                // X logo
                let s = size * 0.35;
                painter.line_segment([Pos2::new(center.x - s, center.y - s), Pos2::new(center.x + s, center.y + s)], Stroke::new(2.5, color));
                painter.line_segment([Pos2::new(center.x + s, center.y - s), Pos2::new(center.x - s, center.y + s)], Stroke::new(2.5, color));
            }
            FooterIcon::Github => {
                // Circle with dot (simplified octocat)
                painter.circle_stroke(center, size * 0.35, Stroke::new(2.0, color));
                painter.circle_filled(center, size * 0.12, color);
            }
            FooterIcon::Whitepaper => {
                // Document icon
                let s = size * 0.3;
                painter.rect_stroke(Rect::from_center_size(center, Vec2::new(s * 1.3, s * 1.7)), 2.0, Stroke::new(1.5, color));
                // Lines inside document
                let line_y1 = center.y - s * 0.4;
                let line_y2 = center.y;
                let line_y3 = center.y + s * 0.4;
                let lx = s * 0.4;
                painter.line_segment([Pos2::new(center.x - lx, line_y1), Pos2::new(center.x + lx, line_y1)], Stroke::new(1.0, color));
                painter.line_segment([Pos2::new(center.x - lx, line_y2), Pos2::new(center.x + lx, line_y2)], Stroke::new(1.0, color));
                painter.line_segment([Pos2::new(center.x - lx, line_y3), Pos2::new(center.x + lx * 0.5, line_y3)], Stroke::new(1.0, color));
            }
            FooterIcon::Audit => {
                // Shield icon
                let s = size * 0.3;
                let pts = vec![
                    Pos2::new(center.x, center.y - s),
                    Pos2::new(center.x + s * 0.7, center.y - s * 0.35),
                    Pos2::new(center.x + s * 0.7, center.y + s * 0.15),
                    Pos2::new(center.x, center.y + s),
                    Pos2::new(center.x - s * 0.7, center.y + s * 0.15),
                    Pos2::new(center.x - s * 0.7, center.y - s * 0.35),
                ];
                painter.add(egui::Shape::convex_polygon(pts, Color32::TRANSPARENT, Stroke::new(1.5, color)));
                // Checkmark inside
                painter.line_segment([Pos2::new(center.x - s * 0.25, center.y), Pos2::new(center.x - s * 0.05, center.y + s * 0.2)], Stroke::new(1.5, color));
                painter.line_segment([Pos2::new(center.x - s * 0.05, center.y + s * 0.2), Pos2::new(center.x + s * 0.25, center.y - s * 0.15)], Stroke::new(1.5, color));
            }
            FooterIcon::KongSwap => {
                // Crown/K shape for Kong
                let s = size * 0.35;
                // K shape
                painter.line_segment([Pos2::new(center.x - s * 0.5, center.y - s), Pos2::new(center.x - s * 0.5, center.y + s)], Stroke::new(2.0, color));
                painter.line_segment([Pos2::new(center.x - s * 0.5, center.y), Pos2::new(center.x + s * 0.5, center.y - s)], Stroke::new(2.0, color));
                painter.line_segment([Pos2::new(center.x - s * 0.5, center.y), Pos2::new(center.x + s * 0.5, center.y + s)], Stroke::new(2.0, color));
            }
            FooterIcon::IcpSwap => {
                // Swap arrows icon
                let s = size * 0.3;
                // Right arrow
                painter.line_segment([Pos2::new(center.x - s, center.y - s * 0.4), Pos2::new(center.x + s, center.y - s * 0.4)], Stroke::new(2.0, color));
                painter.line_segment([Pos2::new(center.x + s * 0.5, center.y - s * 0.8), Pos2::new(center.x + s, center.y - s * 0.4)], Stroke::new(2.0, color));
                painter.line_segment([Pos2::new(center.x + s * 0.5, center.y), Pos2::new(center.x + s, center.y - s * 0.4)], Stroke::new(2.0, color));
                // Left arrow
                painter.line_segment([Pos2::new(center.x + s, center.y + s * 0.4), Pos2::new(center.x - s, center.y + s * 0.4)], Stroke::new(2.0, color));
                painter.line_segment([Pos2::new(center.x - s * 0.5, center.y), Pos2::new(center.x - s, center.y + s * 0.4)], Stroke::new(2.0, color));
                painter.line_segment([Pos2::new(center.x - s * 0.5, center.y + s * 0.8), Pos2::new(center.x - s, center.y + s * 0.4)], Stroke::new(2.0, color));
            }
            FooterIcon::DexScreener => {
                // Chart/graph icon
                let s = size * 0.35;
                // Axes
                painter.line_segment([Pos2::new(center.x - s, center.y + s), Pos2::new(center.x - s, center.y - s)], Stroke::new(1.5, color));
                painter.line_segment([Pos2::new(center.x - s, center.y + s), Pos2::new(center.x + s, center.y + s)], Stroke::new(1.5, color));
                // Candlestick bars
                painter.line_segment([Pos2::new(center.x - s * 0.5, center.y + s * 0.5), Pos2::new(center.x - s * 0.5, center.y - s * 0.2)], Stroke::new(3.0, color));
                painter.line_segment([Pos2::new(center.x, center.y + s * 0.5), Pos2::new(center.x, center.y - s * 0.6)], Stroke::new(3.0, color));
                painter.line_segment([Pos2::new(center.x + s * 0.5, center.y + s * 0.5), Pos2::new(center.x + s * 0.5, center.y)], Stroke::new(3.0, color));
            }
            FooterIcon::IcpTokens => {
                // Coin/token icon - circle with inner detail
                let s = size * 0.35;
                painter.circle_stroke(center, s, Stroke::new(2.0, color));
                // Inner ring
                painter.circle_stroke(center, s * 0.6, Stroke::new(1.5, color));
                // Center dot
                painter.circle_filled(center, s * 0.15, color);
            }
        }
    }
}

fn open_url(url: &str) {
    if let Some(w) = web_sys::window() {
        let _ = w.open_with_url_and_target(url, "_blank");
    }
}

impl eframe::App for AlexandriaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.screen_rect();
        let width = screen_rect.width();
        let typo = Typography::for_width(width);
        let colors = self.colors;
        let max_content_width = 1100.0_f32.min(width - 32.0);

        // Calculate delta time for smooth animations
        let current_time = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now() / 1000.0)
            .unwrap_or(0.0);
        let relative_time = current_time - self.start_time;
        let dt = (current_time - self.last_frame_time) as f32;
        let dt = dt.clamp(0.001, 0.1); // Clamp to avoid jumps
        self.last_frame_time = current_time;

        let mut style = (*ctx.style()).clone();
        style.visuals.widgets.noninteractive.bg_fill = Color32::TRANSPARENT;
        style.visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
        style.spacing.item_spacing = Vec2::new(8.0, 8.0);
        ctx.set_style(style);

        ctx.layer_painter(egui::LayerId::background()).rect_filled(screen_rect, 0.0, colors.bg_primary);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.set_max_width(max_content_width);
                        ui.add_space(40.0);

                        // Hero with typewriter effect
                        let display_text = self.typewriter.update(relative_time).to_string();
                        let cursor = if self.typewriter.cursor_visible(relative_time) { "|" } else { " " };

                        ui.horizontal(|ui| {
                            let text_galley = ui.fonts(|f| {
                                f.layout_no_wrap(
                                    display_text.clone(),
                                    FontId::new(typo.title_size, FontFamily::Proportional),
                                    colors.text_primary,
                                )
                            });
                            let cursor_galley = ui.fonts(|f| {
                                f.layout_no_wrap(
                                    cursor.to_string(),
                                    FontId::new(typo.title_size, FontFamily::Proportional),
                                    colors.text_muted,
                                )
                            });
                            let total_width = text_galley.size().x + cursor_galley.size().x;
                            let available = ui.available_width();
                            ui.add_space(((available - total_width) / 2.0).max(0.0));
                            ui.label(egui::RichText::new(&display_text).size(typo.title_size).color(colors.text_primary));
                            ui.label(egui::RichText::new(cursor).size(typo.title_size).color(colors.text_muted));
                        });
                        ui.add_space(12.0);
                        ui.label(egui::RichText::new(SUBTITLE_TEXT).size(typo.subtitle_size).color(colors.text_secondary));
                        ui.add_space(40.0);

                        // Request continuous repaint for animation
                        ctx.request_repaint();

                        // Metrics border
                        let r = ui.available_rect_before_wrap();
                        ui.painter().line_segment([Pos2::new(r.left(), r.top()), Pos2::new(r.right(), r.top())], Stroke::new(1.0, colors.border));
                        ui.add_space(20.0);

                        // Metrics with hover glow
                        ui.horizontal(|ui| {
                            let mw = if width < 500.0 { 70.0 } else { 120.0 };
                            let mh = 60.0;
                            let sp = (ui.available_width() - METRICS.len() as f32 * mw) / (METRICS.len() + 1) as f32;
                            for (idx, m) in METRICS.iter().enumerate() {
                                ui.add_space(sp.max(8.0));
                                ui.vertical(|ui| {
                                    ui.set_width(mw);
                                    ui.set_min_height(mh);
                                    ui.vertical_centered(|ui| {
                                        // Update hover state
                                        let hover_rect = Rect::from_min_size(
                                            ui.cursor().min,
                                            Vec2::new(mw, mh)
                                        );
                                        let hover_sense = ui.interact(hover_rect, ui.id().with(("metric", idx)), Sense::hover());
                                        let hover_intensity = self.hover.update_metric(idx, hover_sense.hovered(), dt);

                                        // Draw glow effect behind metric
                                        if hover_intensity > 0.01 {
                                            let glow_color = lerp_color(colors.accent_rust, colors.accent_ember, hover_intensity);
                                            let alpha = (40.0 * hover_intensity) as u8;
                                            let bg = Color32::from_rgba_unmultiplied(glow_color.r(), glow_color.g(), glow_color.b(), alpha);
                                            ui.painter().rect_filled(hover_rect.expand(4.0), Rounding::same(8.0), bg);
                                        }

                                        // Metric value with glow color on hover
                                        let value_color = if hover_intensity > 0.5 {
                                            lerp_color(colors.text_primary, colors.accent_ember, (hover_intensity - 0.5) * 2.0)
                                        } else {
                                            colors.text_primary
                                        };
                                        self.centered_label(ui, m.value, typo.metric_value_size, value_color);
                                        ui.add_space(4.0);
                                        ui.label(egui::RichText::new(m.label.to_uppercase()).size(typo.metric_label_size).color(colors.text_muted));
                                    });
                                });
                            }
                        });
                        ui.add_space(20.0);

                        let r = ui.available_rect_before_wrap();
                        ui.painter().line_segment([Pos2::new(r.left(), r.top()), Pos2::new(r.right(), r.top())], Stroke::new(1.0, colors.border));
                        ui.add_space(30.0);

                        // Products with cyberpunk hover effects
                        for (product_idx, p) in PRODUCTS.iter().enumerate() {
                            ui.scope(|ui| {
                                let row = ui.vertical(|ui| {
                                    ui.add_space(16.0);

                                    // Product header with image
                                    ui.horizontal(|ui| {
                                        // Product logo - construct full URL
                                        let img_size = 40.0;
                                        let full_image_url = format!("{}{}", self.base_url, p.image);
                                        ui.add(
                                            egui::Image::new(full_image_url)
                                                .fit_to_exact_size(Vec2::splat(img_size))
                                                .rounding(4.0)
                                        );
                                        ui.add_space(12.0);

                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new(p.name).size(typo.product_name_size).color(colors.text_primary));
                                            ui.add_space(4.0);
                                            ui.horizontal_wrapped(|ui| {
                                                for t in p.tags {
                                                    let g = ui.fonts(|f| f.layout_no_wrap(t.to_uppercase(), FontId::new(typo.tag_size, FontFamily::Proportional), colors.tag_text));
                                                    let sz = g.size() + Vec2::new(10.0, 4.0);
                                                    let (tr, tag_resp) = ui.allocate_exact_size(sz, Sense::hover());

                                                    // Tag glow on hover
                                                    let tag_glow = if tag_resp.hovered() { 1.0 } else { 0.0 };
                                                    if tag_glow > 0.0 {
                                                        self.draw_tag_glow(ui.painter(), tr, tag_glow, relative_time);
                                                    }

                                                    ui.painter().rect_filled(tr, Rounding::same(3.0), colors.tag_bg);
                                                    ui.painter().galley(tr.min + Vec2::new(5.0, 2.0), g, colors.tag_text);
                                                }
                                            });
                                        });
                                    });

                                    ui.add_space(12.0);
                                    ui.label(egui::RichText::new(p.tagline).size(typo.tagline_size).color(colors.text_primary));
                                    ui.add_space(8.0);
                                    ui.label(egui::RichText::new(p.description).size(typo.description_size).color(colors.text_secondary));
                                    ui.add_space(16.0);

                                    let br = ui.available_rect_before_wrap();
                                    ui.painter().line_segment([Pos2::new(br.left(), br.top()), Pos2::new(br.right(), br.top())], Stroke::new(1.0, colors.border));
                                });

                                let card_rect = row.response.rect;
                                let click = ui.interact(card_rect, ui.id().with(p.name), Sense::click());
                                let is_hovered = click.hovered();

                                // Update hover animation state
                                let hover_intensity = self.hover.update_hover(product_idx, is_hovered, dt);
                                let scanline_pos = self.hover.update_scanline(product_idx, is_hovered, dt);

                                // Draw cyberpunk effects (in reverse order for proper layering)
                                // 1. Background glow
                                self.draw_hover_bg(ui.painter(), card_rect, hover_intensity);

                                // 2. Glowing border
                                self.draw_glow_border(ui.painter(), card_rect, hover_intensity, relative_time);

                                // 3. Animated scanline sweep
                                self.draw_scanline(ui.painter(), card_rect, scanline_pos, hover_intensity);

                                if click.clicked() { open_url(p.url); }
                                if is_hovered { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
                            });
                        }

                        ui.add_space(40.0);

                        // Footer with cyberpunk glow and rusty tooltips
                        ui.horizontal(|ui| {
                            let isz = 28.0;
                            let tw = FOOTER_LINKS.len() as f32 * (isz + 20.0);
                            ui.add_space(((ui.available_width() - tw) / 2.0).max(0.0));
                            for l in FOOTER_LINKS {
                                let (r, rsp) = ui.allocate_exact_size(Vec2::splat(isz + 8.0), Sense::click());
                                let hov = rsp.hovered();

                                // Cyberpunk glow on hover
                                if hov {
                                    // Animated glow color
                                    let phase = (relative_time * 4.0) as f32;
                                    let glow_t = phase.sin() * 0.5 + 0.5;
                                    let glow_color = lerp_color(colors.accent_copper, colors.accent_ember, glow_t);

                                    // Multiple glow layers
                                    for (alpha, expand) in [(30, 12.0), (50, 8.0), (80, 4.0)] {
                                        let glow = Color32::from_rgba_unmultiplied(
                                            glow_color.r(), glow_color.g(), glow_color.b(), alpha
                                        );
                                        ui.painter().circle_stroke(r.center(), (isz / 2.0) + expand, Stroke::new(2.0, glow));
                                    }
                                }

                                let col = if hov { colors.accent_ember } else { colors.text_muted };
                                let sc = if hov { 1.15 } else { 1.0 };
                                self.render_footer_icon(ui.painter(), l.icon, r.center(), isz * sc, col);

                                // Rusty tooltip on hover
                                rsp.clone().on_hover_ui_at_pointer(|ui| {
                                    // Style the tooltip with rust theme
                                    let tooltip_frame = egui::Frame::none()
                                        .fill(Color32::from_rgb(20, 20, 20))
                                        .stroke(Stroke::new(1.5, colors.accent_rust))
                                        .rounding(Rounding::same(4.0))
                                        .inner_margin(egui::Margin::symmetric(12.0, 8.0));

                                    tooltip_frame.show(ui, |ui| {
                                        ui.label(
                                            egui::RichText::new(l.title)
                                                .size(13.0)
                                                .color(colors.accent_ember)
                                        );
                                    });
                                });

                                if rsp.clicked() { open_url(l.url); }
                                if hov { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
                                ui.add_space(12.0);
                            }
                        });

                        ui.add_space(60.0);
                    });
                });
            });
    }
}

// Logging helper for debugging
macro_rules! console_log {
    ($($arg:tt)*) => {{
        web_sys::console::log_1(&format!("[RUST] {}", format!($($arg)*)).into());
    }};
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log!("WASM module loaded successfully");

    wasm_bindgen_futures::spawn_local(async {
        console_log!("Starting application initialization...");

        let doc = web_sys::window().expect("No window").document().expect("No document");
        console_log!("Got document");

        let canvas = doc.get_element_by_id("canvas").expect("No canvas").dyn_into::<web_sys::HtmlCanvasElement>().expect("Not a canvas");
        console_log!("Got canvas element");

        console_log!("Starting eframe WebRunner...");
        let result = eframe::WebRunner::new().start(canvas, eframe::WebOptions::default(), Box::new(|cc| {
            console_log!("Creating AlexandriaApp...");
            Ok(Box::new(AlexandriaApp::new(cc)))
        })).await;

        match result {
            Ok(_) => console_log!("Application started successfully"),
            Err(e) => console_log!("Failed to start application: {:?}", e),
        }
    });
}
