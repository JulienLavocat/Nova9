@group(2) @binding(0) var base_texture: texture_2d<f32>;
@group(2) @binding(1) var base_sampler: sampler;

@group(2) @binding(2) var details_texture: texture_2d<f32>;
@group(2) @binding(3) var details_sampler: sampler;

@group(2) @binding(4) var emissive_texture: texture_2d<f32>;
@group(2) @binding(5) var emissive_sampler: sampler;

@group(2) @binding(6) var<uniform> details_amount: f32;

@group(2) @binding(7) var<uniform> emissive_color: vec4<f32>;

const WHITE: vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 1.0);

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) uv_emmissive: vec2<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) uv_details: vec2<f32>,
};

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let base_sampled_color = textureSample(base_texture, base_sampler, in.uv);
    let details_sampled_color = textureSample(details_texture, details_sampler, in.uv_details);
    let emissive_sampled_color = textureSample(emissive_texture, emissive_sampler, in.uv_emmissive);

    let details = mix(WHITE, details_sampled_color, details_amount);

    let emissive = emissive_sampled_color * emissive_color;

    return base_sampled_color * details + emissive;
}
