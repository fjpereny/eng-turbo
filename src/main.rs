
use aga8;
use eframe::egui;

mod gas_properties;
mod gui;

struct MyApp {
    // UI States
    show_win_gas_comp: bool,
    show_win_gas_comp_warn: bool,

    // Gas Calculation States
    gas_comp: gas_properties::gas_comp::GasComp,
    composition: aga8::composition::Composition,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {  
            // UI States
            show_win_gas_comp: true,
            show_win_gas_comp_warn: false,

            // Gas Calculation States
            gas_comp: gas_properties::gas_comp::GasComp::default(),
            composition: aga8::composition::Composition::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
        .frame(egui::Frame { 
            inner_margin: egui::Margin::same(10),
            ..Default::default()
        })
        .show(ctx, |ui| {
            // ui.style_mut().spacing.window_margin = egui::Margin::symmetric(20, 20);
            if self.show_win_gas_comp {
                gui::win_gas_comp::gas_comp_window(self, ctx, ui);
            }
        });
    }
}


fn main() {

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Eng Turbo", 
        native_options, 
        Box::new(|_cc| Ok(Box::new(MyApp::default())))
    );   
}
