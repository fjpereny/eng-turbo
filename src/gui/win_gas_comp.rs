
use eframe::egui::{self, Color32, Ui, Vec2};
use aga8::composition::Composition;
use crate::MyApp;
use crate::gui::win_gas_comp_warn;
use crate::gui::plots::pie_chart;

pub fn gas_comp_window(app: &mut MyApp, ctx: &eframe::egui::Context, _ui: &mut Ui) {
    
    egui::Window::new("Gas Composition")
        .open(&mut app.show_win_gas_comp)
        .resizable(false)
        .show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut app.gas_comp.Argon, 0.0..=100.0).text("Argon"));
            ui.add(egui::Slider::new(&mut app.gas_comp.CarbonDioxide, 0.0..=100.0).text("Carbon Dioxide"));
            ui.add(egui::Slider::new(&mut app.gas_comp.CarbonMonoxide, 0.0..=100.0).text("Carbon Monoxide"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Helium, 0.0..=100.0).text("Helium"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Hydrogen, 0.0..=100.0).text("Hydrogen"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Nitrogen, 0.0..=100.0).text("Nitrogen"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Oxygen, 0.0..=100.0).text("Oxygen"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Water, 0.0..=100.0).text("Water"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Methane, 0.0..=100.0).text("Methane"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Ethane, 0.0..=100.0).text("Ethane"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Propane, 0.0..=100.0).text("Propane"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Isobutane, 0.0..=100.0).text("Isobutane"));
            ui.add(egui::Slider::new(&mut app.gas_comp.NButane, 0.0..=100.0).text("n-Butane"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Isopentane, 0.0..=100.0).text("Isopentane"));
            ui.add(egui::Slider::new(&mut app.gas_comp.NPentane, 0.0..=100.0).text("n-Pentane"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Hexane, 0.0..=100.0).text("Hexane"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Heptane, 0.0..=100.0).text("Heptane"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Octane, 0.0..=100.0).text("Octane"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Nonane, 0.0..=100.0).text("Nonane"));
            ui.add(egui::Slider::new(&mut app.gas_comp.Decane, 0.0..=100.0).text("Decane"));
            ui.add(egui::Slider::new(&mut app.gas_comp.HydrogenSulfide, 0.0..=100.0).text("Hydrogen Sulfide"));

            if ui.add(egui::Button::new("Set Composition")).clicked() {
                let mut composition = Composition {
                    argon: app.gas_comp.Argon,
                    carbon_dioxide: app.gas_comp.CarbonDioxide,
                    carbon_monoxide: app.gas_comp.CarbonMonoxide,
                    helium: app.gas_comp.Helium,
                    hydrogen: app.gas_comp.Hydrogen,
                    nitrogen: app.gas_comp.Nitrogen,
                    oxygen: app.gas_comp.Oxygen,
                    water: app.gas_comp.Water,
                    methane: app.gas_comp.Methane,
                    ethane: app.gas_comp.Ethane,
                    propane: app.gas_comp.Propane,
                    isobutane: app.gas_comp.Isobutane,
                    n_butane: app.gas_comp.NButane,
                    isopentane: app.gas_comp.Isopentane,
                    n_pentane: app.gas_comp.NPentane,
                    hexane: app.gas_comp.Hexane,
                    heptane: app.gas_comp.Heptane,
                    octane: app.gas_comp.Octane,
                    nonane: app.gas_comp.Nonane,
                    decane: app.gas_comp.Decane,
                    hydrogen_sulfide: app.gas_comp.HydrogenSulfide,

                };
                let _ = composition.normalize();
                
                if composition.check().is_err() {
                    app.show_win_gas_comp_warn = true;
                } else {
                    app.composition = composition;
                    app.show_win_gas_comp_warn = false;
                }
            };

            let desired_size = Vec2::new(250.0, 250.0);
            let radius = 100.0;
            let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
            ui.painter().debug_rect(rect, Color32::MAGENTA, "Debug Rectangle");

            let mut data = vec![];
            if app.gas_comp.Argon > 0.0 {
                data.push(
                    ("Argon", app.gas_comp.Argon as f32, Color32::GREEN)
                );
            }
            if app.gas_comp.CarbonDioxide > 0.0 {
                data.push(
                    ("CO2", app.gas_comp.CarbonDioxide as f32, Color32::GRAY),
                );
            }
            if app.gas_comp.CarbonMonoxide > 0.0 {
                data.push(
                    ("CO", app.gas_comp.CarbonMonoxide as f32, Color32::RED),
                );
            }
            if app.gas_comp.Helium > 0.0 {
                data.push(
                    ("Helium", app.gas_comp.Helium as f32, Color32::LIGHT_BLUE),
                );
            }
            if app.gas_comp.Hydrogen > 0.0 {
                data.push(
                    ("Hydrogen", app.gas_comp.Hydrogen as f32, Color32::BLUE),
                );
            }
            if app.gas_comp.Nitrogen > 0.0 {
                data.push(
                    ("Nitrogen", app.gas_comp.Nitrogen as f32, Color32::LIGHT_GREEN),
                );
            }
            if app.gas_comp.Oxygen > 0.0 {
                data.push(
                ("Oxygen", app.gas_comp.Oxygen as f32, Color32::LIGHT_RED),
                );
            }
            
            pie_chart::draw_pie_chart(ui, radius, &data, rect);
            
        });
    if app.show_win_gas_comp_warn {
    win_gas_comp_warn::gas_comp_window(app, ctx);
    }
}

