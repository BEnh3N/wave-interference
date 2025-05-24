use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use super::ShaderParams;

const LIGHT_PATTERN_SHADER_PATH: &str = "shaders/light_pattern.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LightPatternMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(1)]
    pub params: ShaderParams,
}

impl Material for LightPatternMaterial {
    fn fragment_shader() -> ShaderRef {
        LIGHT_PATTERN_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
