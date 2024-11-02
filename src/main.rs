use core::f32;

use eframe::{run_native, App, CreationContext, NativeOptions};
use egui::{Id, Label, Layout, RichText, TextEdit, TopBottomPanel};

mod calculator;

#[derive(Default)]
struct DeskCalc {
    input_text: String,
    out: String
}

impl DeskCalc {

    fn new(_cc: &CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for DeskCalc {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Bottom panel containing input and output
        TopBottomPanel::bottom(Id::new("output_display"))
            .show(ctx, |ui| {

                // Add text editor
                let response = ui.add(
                    TextEdit::singleline(&mut self.input_text)
                    .hint_text("Enter an expression...")
                    .frame(false)
                    .desired_width(f32::INFINITY)
                );
                if response.changed() {
                    // Calculate output given response and set output buffer
                    // Errors are mapped to a string representation which still goes in the output buffer
                    self.out = calculator::calculate(&self.input_text).unwrap_or_else(|e| {
                        e.as_string()
                    });
                }
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    // TODO: send to log
                }

                ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                    // Add output line
                    ui.add(
                        Label::new(RichText::new(&self.out).heading().strong())
                    );
                });
            });
        
        // Add TopBottomPanel for menu here

        // Add CentralPanel showing log here

    }
}

fn main() {
    let win_option = NativeOptions::default();
    run_native("DeskCalc", win_option, Box::new(|cc| Ok(Box::new(DeskCalc::new(cc)))))
        .expect("Failed to set up window");
}
