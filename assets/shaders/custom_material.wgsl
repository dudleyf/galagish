#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct CustomMaterial {
    color: vec4<f32>,
    tile: f32,
    time: f32,
};

@group(2) @binding(0) var<uniform> material: CustomMaterial;
@group(2) @binding(1) var base_color_texture: texture_2d<f32>;
@group(2) @binding(2) var base_color_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput
) -> @location(0) vec4<f32> {
    var tiled_uv: vec2<f32>;
    tiled_uv = mesh.uv;
    if(material.tile > 0.0) {
        var tiled_uv_x: f32;
        var tiled_uv_y: f32;
        tiled_uv_x = fract(mesh.uv.x * 10.0);
        tiled_uv_y = fract(mesh.uv.y * 5.0 - material.time);
        tiled_uv = vec2(tiled_uv_x, tiled_uv_y);
    }
    return textureSample(base_color_texture, base_color_sampler, tiled_uv);
}
