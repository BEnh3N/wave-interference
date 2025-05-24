use bevy::{prelude::*, render::render_resource::ShaderType};
use interference::InterferenceMaterial;
use light_pattern::LightPatternMaterial;

pub mod interference;
pub mod light_pattern;

#[derive(Resource, ShaderType, Debug, Clone)]
pub struct ShaderParams {
    pub num_slits: u32,
    pub spacing: f32,
    pub wavelength: f32,
    pub velocity: f32,
    pub damping: f32,
}

impl Default for ShaderParams {
    fn default() -> Self {
        let num_slits = 2;
        let wavelength = 0.2;
        let spacing = wavelength * 3.0;
        ShaderParams {
            num_slits,
            spacing,
            wavelength,
            velocity: 0.15,
            damping: 0.15,
        }
    }
}

fn update_interference_material(
    time: Res<Time>,
    params: Res<ShaderParams>,
    interference_plane_query: Query<&MeshMaterial3d<InterferenceMaterial>>,
    mut interference_material_query: ResMut<Assets<InterferenceMaterial>>,
) {
    let interference_plane = interference_plane_query.single().unwrap();
    let interference_material = interference_material_query
        .get_mut(interference_plane.id())
        .unwrap();
    interference_material.time = time.elapsed_secs();
    interference_material.params = params.clone();
}

fn update_light_material(
    time: Res<Time>,
    params: Res<ShaderParams>,
    light_plane_query: Query<&MeshMaterial3d<LightPatternMaterial>>,
    mut light_material_query: ResMut<Assets<LightPatternMaterial>>,
) {
    let light_plane = light_plane_query.single().unwrap();
    let light_material = light_material_query.get_mut(light_plane.id()).unwrap();
    light_material.time = time.elapsed_secs();
    light_material.params = params.clone();
}

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialPlugin::<InterferenceMaterial>::default(),
            MaterialPlugin::<LightPatternMaterial>::default(),
        ))
        .add_systems(
            Update,
            (update_interference_material, update_light_material),
        );
    }
}
