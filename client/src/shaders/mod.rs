use bevy::{prelude::*, render::render_resource::AsBindGroup};

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<SpaceStationMaterial>::default());
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SpaceStationMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,

    #[texture(2)]
    #[sampler(3)]
    pub details: Handle<Image>,

    #[texture(4)]
    #[sampler(5)]
    pub emissive: Handle<Image>,

    #[uniform(6)]
    pub details_amount: f32,

    #[uniform(7)]
    pub emissive_color: Vec4,
}

impl Material for SpaceStationMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/spacestation.wgsl".into()
    }
}
