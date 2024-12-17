#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> material_size: vec2<f32>;
@group(2) @binding(2) var<uniform> frame_width: f32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4f {
    let left = frame_width / material_size.x;
    let top = frame_width / material_size.y;
    let right = 1.0 - left;
    let bottom = 1.0 - top;

    if (mesh.uv.x < left || mesh.uv.x > right || mesh.uv.y < top || mesh.uv.y > bottom)  {
        return material_color;
    }

    return vec4f(material_color.rgb, 0.0);
}
