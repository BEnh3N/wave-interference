use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use super::ShaderParams;

const INTERFERENCE_SHADER_PATH: &str = "shaders/interference.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct InterferenceMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(1)]
    pub params: ShaderParams,
}

impl Material for InterferenceMaterial {
    fn fragment_shader() -> ShaderRef {
        INTERFERENCE_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

impl InterferenceMaterial {
    pub fn with_params(params: ShaderParams) -> InterferenceMaterial {
        InterferenceMaterial { time: 0.0, params }
    }
}
