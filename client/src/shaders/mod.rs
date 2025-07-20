use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::AsBindGroup,
};

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, DetailedMaterialExtension>,
        >::default());
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct DetailedMaterialExtension {
    // Starting from 100 to avoid conflicts with StandardMaterial
    // See: https://bevy.org/examples/shaders/extended-material/
    #[texture(100)]
    #[sampler(101)]
    pub texture: Handle<Image>,

    #[texture(102)]
    #[sampler(103)]
    pub details: Handle<Image>,

    #[uniform(104)]
    pub details_amount: f32,
}

impl MaterialExtension for DetailedMaterialExtension {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/spacestation.wgsl".into()
    }
}
