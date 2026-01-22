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
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            bg_primary: Color32::from_rgb(0, 0, 0),
            text_primary: Color32::from_rgb(255, 255, 255),
            text_secondary: Color32::from_rgba_unmultiplied(255, 255, 255, 178),
            text_muted: Color32::from_rgba_unmultiplied(255, 255, 255, 128),
            border: Color32::from_rgba_unmultiplied(255, 255, 255, 26),
            tag_bg: Color32::from_rgba_unmultiplied(124, 58, 237, 38),
            tag_text: Color32::from_rgb(167, 139, 250),
        }
    }
}

fn gradient_color(t: f32) -> Color32 {
    let r = (124.0 + (37.0 - 124.0) * t) as u8;
    let g = (58.0 + (99.0 - 58.0) * t) as u8;
    let b = (237.0 + (235.0 - 237.0) * t) as u8;
    Color32::from_rgb(r, g, b)
}

struct Typography { title_size: f32, subtitle_size: f32, metric_value_size: f32, metric_label_size: f32, product_name_size: f32, tagline_size: f32, description_size: f32, tag_size: f32 }

impl Typography {
    fn for_width(width: f32) -> Self {
        if width < 480.0 { Self { title_size: 28.0, subtitle_size: 14.0, metric_value_size: 18.0, metric_label_size: 9.0, product_name_size: 15.0, tagline_size: 12.0, description_size: 11.0, tag_size: 8.0 } }
        else if width < 768.0 { Self { title_size: 36.0, subtitle_size: 16.0, metric_value_size: 22.0, metric_label_size: 10.0, product_name_size: 16.0, tagline_size: 13.0, description_size: 12.0, tag_size: 9.0 } }
        else { Self { title_size: 52.0, subtitle_size: 20.0, metric_value_size: 26.0, metric_label_size: 11.0, product_name_size: 18.0, tagline_size: 14.0, description_size: 13.0, tag_size: 9.0 } }
    }
}

#[derive(Clone)] struct Product { name: &'static str, tagline: &'static str, description: &'static str, url: &'static str, tags: &'static [&'static str] }
#[derive(Clone)] struct Metric { value: &'static str, label: &'static str }
#[derive(Clone)] struct FooterLink { url: &'static str, icon: FooterIcon }
#[derive(Clone, Copy)] enum FooterIcon { Twitter, Github, Whitepaper, Audit }

const TITLE_TEXT: &str = "Building ICP's Great City";
const PRODUCTS: &[Product] = &[
    Product { name: "lbry.app", tagline: "The library and city center.", description: "Decentralized archival platform.", url: "https://lbry.app", tags: &["DeFi"] },
    Product { name: "CycleScan", tagline: "Cycle burn leaderboard.", description: "Tracks cycle consumption.", url: "https://xknwi-uaaaa-aaaak-qu4oq-cai.icp0.io", tags: &["Infrastructure"] },
    Product { name: "IPG", tagline: "Payment gateway.", description: "Accept BTC/ICP.", url: "https://62rey-3aaaa-aaaag-acjla-cai.icp0.io", tags: &["Payments"] },
    Product { name: "OpenHouse", tagline: "Decentralized casino.", description: "Provably fair.", url: "https://openhouse.games", tags: &["Gaming"] },
    Product { name: "DAOPad", tagline: "LLCs as contracts.", description: "Hackathon winner.", url: "https://daopad.org", tags: &["DAO"] },
    Product { name: "KongLocker", tagline: "Lock liquidity.", description: "LP locking.", url: "https://konglocker.com", tags: &["DeFi"] },
];
const METRICS: &[Metric] = &[ Metric { value: "50K+", label: "NFTs" }, Metric { value: "7", label: "dApps" }, Metric { value: "WCHL2025", label: "Winner" }, Metric { value: "100%", label: "Fair" } ];
const FOOTER_LINKS: &[FooterLink] = &[ FooterLink { url: "https://x.com/alexandria_lbry", icon: FooterIcon::Twitter }, FooterLink { url: "https://github.com/AlexandriaDAO", icon: FooterIcon::Github } ];

pub struct AlexandriaApp { colors: ColorPalette }
impl Default for AlexandriaApp { fn default() -> Self { Self { colors: ColorPalette::default() } } }

impl AlexandriaApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self { Self::default() }

    fn gradient_label(&self, ui: &mut egui::Ui, text: &str, font_size: f32) {
        if text.is_empty() { return; }
        let font_id = FontId::new(font_size, FontFamily::Proportional);
        let char_count = text.chars().count();
        let mut job = egui::text::LayoutJob::default();
        job.halign = egui::Align::Center;
        for (i, c) in text.chars().enumerate() {
            let t = if char_count > 1 { i as f32 / (char_count - 1) as f32 } else { 0.5 };
            job.append(&c.to_string(), 0.0, egui::TextFormat { font_id: font_id.clone(), color: gradient_color(t), ..Default::default() });
        }
        let galley = ui.fonts(|f| f.layout_job(job));
        let (rect, _) = ui.allocate_exact_size(galley.size(), Sense::hover());
        ui.painter().galley(rect.min, galley, Color32::WHITE);
    }

    fn render_footer_icon(&self, painter: &egui::Painter, icon: FooterIcon, center: Pos2, size: f32, color: Color32) {
        match icon {
            FooterIcon::Twitter => { let s = size * 0.35; painter.line_segment([Pos2::new(center.x - s, center.y - s), Pos2::new(center.x + s, center.y + s)], Stroke::new(2.5, color)); painter.line_segment([Pos2::new(center.x + s, center.y - s), Pos2::new(center.x - s, center.y + s)], Stroke::new(2.5, color)); }
            FooterIcon::Github => { painter.circle_stroke(center, size * 0.35, Stroke::new(2.0, color)); painter.circle_filled(center, size * 0.12, color); }
            FooterIcon::Whitepaper => { let s = size * 0.3; painter.rect_stroke(Rect::from_center_size(center, Vec2::new(s * 1.3, s * 1.7)), 2.0, Stroke::new(1.5, color)); }
            FooterIcon::Audit => { let s = size * 0.3; let pts = vec![Pos2::new(center.x, center.y - s), Pos2::new(center.x + s * 0.7, center.y - s * 0.35), Pos2::new(center.x + s * 0.7, center.y + s * 0.15), Pos2::new(center.x, center.y + s), Pos2::new(center.x - s * 0.7, center.y + s * 0.15), Pos2::new(center.x - s * 0.7, center.y - s * 0.35)]; painter.add(egui::Shape::convex_polygon(pts, Color32::TRANSPARENT, Stroke::new(1.5, color))); }
        }
    }
}

fn open_url(url: &str) { if let Some(w) = web_sys::window() { let _ = w.open_with_url_and_target(url, "_blank"); } }

impl eframe::App for AlexandriaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let sr = ctx.screen_rect(); let w = sr.width(); let typo = Typography::for_width(w); let c = self.colors; let mcw = 1100.0_f32.min(w - 32.0);
        let mut style = (*ctx.style()).clone(); style.visuals.widgets.noninteractive.bg_fill = Color32::TRANSPARENT; style.visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT; style.spacing.item_spacing = Vec2::new(8.0, 8.0); ctx.set_style(style);
        ctx.layer_painter(egui::LayerId::background()).rect_filled(sr, 0.0, c.bg_primary);
        egui::CentralPanel::default().frame(egui::Frame::none().fill(Color32::TRANSPARENT)).show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.set_max_width(mcw); ui.add_space(40.0);
                    self.gradient_label(ui, TITLE_TEXT, typo.title_size); ui.add_space(12.0);
                    ui.label(egui::RichText::new("Alexandria DAO").size(typo.subtitle_size).color(c.text_secondary)); ui.add_space(40.0);
                    let r = ui.available_rect_before_wrap(); ui.painter().line_segment([Pos2::new(r.left(), r.top()), Pos2::new(r.right(), r.top())], Stroke::new(1.0, c.border)); ui.add_space(20.0);
                    ui.horizontal(|ui| { let mw = if w < 500.0 { 70.0 } else { 120.0 }; let sp = (ui.available_width() - METRICS.len() as f32 * mw) / (METRICS.len() + 1) as f32; for m in METRICS { ui.add_space(sp.max(8.0)); ui.vertical(|ui| { ui.set_width(mw); ui.vertical_centered(|ui| { self.gradient_label(ui, m.value, typo.metric_value_size); ui.add_space(4.0); ui.label(egui::RichText::new(m.label.to_uppercase()).size(typo.metric_label_size).color(c.text_muted)); }); }); } });
                    ui.add_space(20.0); let r = ui.available_rect_before_wrap(); ui.painter().line_segment([Pos2::new(r.left(), r.top()), Pos2::new(r.right(), r.top())], Stroke::new(1.0, c.border)); ui.add_space(30.0);
                    for p in PRODUCTS { ui.scope(|ui| { let row = ui.vertical(|ui| { ui.add_space(16.0); ui.label(egui::RichText::new(p.name).size(typo.product_name_size).color(c.text_primary)); ui.add_space(8.0); ui.horizontal_wrapped(|ui| { for t in p.tags { let g = ui.fonts(|f| f.layout_no_wrap(t.to_uppercase(), FontId::new(typo.tag_size, FontFamily::Proportional), c.tag_text)); let sz = g.size() + Vec2::new(10.0, 4.0); let (tr, _) = ui.allocate_exact_size(sz, Sense::hover()); ui.painter().rect_filled(tr, Rounding::same(3.0), c.tag_bg); ui.painter().galley(tr.min + Vec2::new(5.0, 2.0), g, c.tag_text); } }); ui.add_space(12.0); ui.label(egui::RichText::new(p.tagline).size(typo.tagline_size).color(c.text_primary)); ui.add_space(8.0); ui.label(egui::RichText::new(p.description).size(typo.description_size).color(c.text_secondary)); ui.add_space(16.0); let br = ui.available_rect_before_wrap(); ui.painter().line_segment([Pos2::new(br.left(), br.top()), Pos2::new(br.right(), br.top())], Stroke::new(1.0, c.border)); }); let clk = ui.interact(row.response.rect, ui.id().with(p.name), Sense::click()); if clk.clicked() { open_url(p.url); } if clk.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); } }); }
                    ui.add_space(40.0);
                    ui.horizontal(|ui| { let isz = 28.0; let tw = FOOTER_LINKS.len() as f32 * (isz + 20.0); ui.add_space(((ui.available_width() - tw) / 2.0).max(0.0)); for l in FOOTER_LINKS { let (r, rsp) = ui.allocate_exact_size(Vec2::splat(isz + 8.0), Sense::click()); let hov = rsp.hovered(); let col = if hov { c.text_primary } else { c.text_muted }; let sc = if hov { 1.1 } else { 1.0 }; self.render_footer_icon(ui.painter(), l.icon, r.center(), isz * sc, col); if rsp.clicked() { open_url(l.url); } if hov { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); } ui.add_space(12.0); } });
                    ui.add_space(60.0);
                });
            });
        });
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async {
        let doc = web_sys::window().expect("No window").document().expect("No document");
        let canvas = doc.get_element_by_id("canvas").expect("No canvas").dyn_into::<web_sys::HtmlCanvasElement>().expect("Not a canvas");
        let _ = eframe::WebRunner::new().start(canvas, eframe::WebOptions::default(), Box::new(|cc| Ok(Box::new(AlexandriaApp::new(cc))))).await;
    });
}
