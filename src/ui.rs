use bevy::prelude::*;
use bevy_egui::{
    EguiContextPass, EguiContexts, EguiPlugin,
    egui::{self, DragValue, Slider},
};

use crate::shaders::{ShaderParams, light_pattern::LightPatternMaterial};

pub fn ui(
    mut contexts: EguiContexts,
    mut params: ResMut<ShaderParams>,
    mut transform: Query<&mut Transform, With<MeshMaterial3d<LightPatternMaterial>>>,
) {
    let mut transform = transform.single_mut().unwrap();

    egui::Window::new("Parameters").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Large").clicked() {
                params.num_slits = 32;
                params.spacing = 0.25;
                params.wavelength = 0.1;
                params.damping = 0.003;
                transform.translation.z = -2.0;
            }
            if ui.button("Double Slit").clicked() {
                params.num_slits = 2;
                params.spacing = 0.60;
                params.wavelength = 0.20;
                params.damping = 0.15;
                transform.translation.z = -2.0;
            }
            if ui.button("Realistic...").clicked() {
                params.num_slits = 2;
                params.spacing = 0.1;
                params.wavelength = 0.01;
                params.damping = 0.0;
                transform.translation.z = -128.0;
            }
        });
        egui::Grid::new("params_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.monospace("# of Slits");
                ui.add(DragValue::new(&mut params.num_slits).range(1..=100));
                ui.end_row();

                ui.monospace("Spacing");
                ui.add(Slider::new(&mut params.spacing, 0.0..=1.0));
                ui.end_row();

                ui.monospace("Wavelength");
                ui.add(Slider::new(&mut params.wavelength, 0.0..=1.0));
                ui.end_row();

                ui.monospace("Velocity");
                ui.add(Slider::new(&mut params.velocity, 0.0..=1.0));
                ui.end_row();

                ui.monospace("Damping");
                ui.add(Slider::new(&mut params.damping, 0.0..=0.5));
                ui.end_row();

                ui.monospace("Plane z");
                ui.add(Slider::new(&mut transform.translation.z, -128.0..=0.0));
                ui.end_row();
            })
    });
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_systems(EguiContextPass, ui);
    }
}
