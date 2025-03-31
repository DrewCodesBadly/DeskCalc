#![windows_subsystem = "windows"]

use core::f32;
use config::{Config, Map, Value};
use directories::ProjectDirs;
use eframe::{run_native, App, CreationContext, NativeOptions};
use egui::{CentralPanel, Color32, FontFamily, FontId, Id, Label, Layout, RichText, Style, TextEdit, TopBottomPanel};
use log::Log;

mod calculator;
mod log;

#[derive(Default)]
struct DeskCalc {
    input_text: String,
    out: String,
    log: Log,
}

impl DeskCalc {
    fn new(_cc: &CreationContext<'_>) -> Self {
        DeskCalc {
            log: Log::new(),
            ..Default::default()
        }
    }
}

impl App for DeskCalc {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Bottom panel containing input and output
        TopBottomPanel::bottom(Id::new("output_display")).show(ctx, |ui| {
            // Add text editor
            let response = ui.add(
                TextEdit::singleline(&mut self.input_text)
                    .hint_text("Enter an expression...")
                    .frame(false)
                    .desired_width(f32::INFINITY),
            );

            // Clear text box when escape pressed
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.input_text.clear();
                response.request_focus();
                self.out = String::from("...");
            }

            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if response.lost_focus() {
                    self.out = calculator::calculate_assign(&self.input_text, &mut self.log);
                    self.log.push_results(&self.input_text, &self.out);
                    self.input_text.clear();
                    self.out.clear();
                }
    
                // Move focus back to text input - can also be used as a shortcut to jump to text
                // box
                response.request_focus();
            } else if response.changed() {
                // Calculate output given response and set output buffer
                self.out = calculator::calculate(&self.input_text, &self.log);
            }

            ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                // Add output line
                ui.add(Label::new(RichText::new(&self.out).heading().strong()));
            });
        });

        // Add TopBottomPanel for menu here

        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::bottom_up(egui::Align::Min), |ui| {
                for command in self.log.history.iter().rev() {
                    ui.add(Label::new(RichText::new(&command.1)).halign(egui::Align::Max));
                    ui.add(Label::new(
                        RichText::new("\t".to_owned() + &command.0).weak(),
                    ));
                }
            });
        });
    }
}

fn get_config_hex(map: &Map<String, Value>, key: &str) -> Option<Color32> {
    map.get(key).and_then(|s| s.clone().into_string().ok()).and_then(|s| Color32::from_hex(&s).ok())
}

fn main() {
    let project_dirs = ProjectDirs::from("", "DrewCodesBadly", "DeskCalc");
    let config_builder = Config::builder();
    let mut config = Config::default();
    if let Some(dirs) = project_dirs {
        let mut path = dirs.preference_dir().to_path_buf();
        path.push("config.toml");
        config = config_builder.add_source(config::File::from(path)).build().unwrap_or_default();
    }
    let win_option = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_transparent(config.get_bool("use_transparency").unwrap_or(false)),
        ..Default::default()
    };

    // Visual styling according to config - only enabled if colors table has been declared
    let mut style = Style::default(); 
    if let Ok(map) = config.get_table("visuals") {
        // Avoid interfering with panel background
        style.visuals.window_fill = Color32::from_rgba_premultiplied(0, 0, 0, 0);

        let mut font_id = FontId::default();
        // Setting each property
        if let Some(color) = get_config_hex(&map, "background") {
            style.visuals.panel_fill = color;
        }
        if let Some(color) = get_config_hex(&map, "text_color") {
            style.visuals.override_text_color = Some(color);
        }
        if let Some(size) = &map.get("font_size").and_then(|v| v.clone().into_float().ok()) {
            font_id.size = *size as f32;
        }
        if let Some(monospace) = &map.get("monospace_fonts").and_then(|v| v.clone().into_bool().ok()) {
            if *monospace {
                font_id.family = FontFamily::Monospace;
            } else {
                font_id.family = FontFamily::Proportional;
            }
        }

        style.override_font_id = Some(font_id);
    }
    

    run_native(
        "DeskCalc",
        win_option,
        Box::new(|cc| {
            cc.egui_ctx.set_style(style);
            Ok(Box::new(DeskCalc::new(cc)))
        }),
    )
    .expect("Failed to set up window");
}
