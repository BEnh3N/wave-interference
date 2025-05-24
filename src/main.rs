use std::f32::consts::PI;

use bevy::{prelude::*, window::WindowCloseRequested};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use wave_interference::{
    shaders::{
        ShaderParams, ShaderPlugin, interference::InterferenceMaterial,
        light_pattern::LightPatternMaterial,
    },
    ui::UiPlugin,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FlyCameraPlugin, ShaderPlugin, UiPlugin))
        .init_resource::<ShaderParams>()
        .insert_resource(ClearColor(Color::srgb_u8(0, 0, 0)))
        .add_systems(Startup, setup)
        .add_systems(Update, exit_program)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut interference_materials: ResMut<Assets<InterferenceMaterial>>,
    mut light_pattern_material: ResMut<Assets<LightPatternMaterial>>,
    params: Res<ShaderParams>,
) {
    commands.spawn((Camera3d::default(), FlyCamera::default()));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default())),
        MeshMaterial3d(light_pattern_material.add(LightPatternMaterial {
            time: 0.0,
            params: params.clone(),
        })),
        Transform::from_rotation(Quat::from_rotation_x(PI / 2.0))
            .with_translation(Vec3::new(0.0, 0.25, -0.5))
            .with_scale(Vec3::new(128.0, 1.0, 0.5)),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default())),
        MeshMaterial3d(
            interference_materials.add(InterferenceMaterial::with_params(params.clone())),
        ),
        Transform::from_scale(Vec3::splat(128.0)).with_translation(Vec3::new(0.0, 0.0, -64.0)),
    ));
}

fn exit_program(
    keys: Res<ButtonInput<KeyCode>>,
    mut window_close: EventReader<WindowCloseRequested>,
    mut exit: EventWriter<AppExit>,
) {
    for _ in window_close.read() {
        exit.write(AppExit::Success);
    }

    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
