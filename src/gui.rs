use bevy::prelude::*;
use bevy_egui::egui::panel::Side;
use bevy_egui::egui::Widget;
use bevy_egui::{egui, EguiContext};

pub fn setup_menu(mut egui_context: ResMut<EguiContext>, mut light: ResMut<AmbientLight>) {
    egui::SidePanel::new(Side::Left, "panel").show(egui_context.ctx_mut(), |ui| {
        ui.label("Menu");
        ui.spacing();
        ui.separator();
        ui.label("Light");
        ui.horizontal(|ui| {
            ui.label("Color");
            let mut color = light.color.as_rgba_f32();
            ui.color_edit_button_rgba_premultiplied(&mut color);
            light.color = Color::from(color);
        });
        ui.horizontal(|ui| {
            ui.label("Brightness");
            egui::Slider::new(&mut light.brightness, 0.0..=10.0).ui(ui);
        });
        ui.separator();
    });
}
