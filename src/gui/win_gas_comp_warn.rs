

use eframe::egui;
use crate::MyApp;

pub fn gas_comp_window(app: &mut MyApp, ctx: &eframe::egui::Context) {
    
    egui::Window::new("Composition Error")
        .resizable(false)
        .show(ctx, |ui| {
            ui.add(egui::widgets::Label::new("Invalid gas composition."));

            if ui.add(egui::widgets::Button::new("OK")).clicked() {
                app.show_win_gas_comp_warn = false;
            }
        });
}