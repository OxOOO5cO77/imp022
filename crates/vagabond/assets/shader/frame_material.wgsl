#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> material_size: vec2<f32>;
@group(2) @binding(2) var<uniform> frame_width: f32;
@group(2) @binding(3) var<uniform> dash_size: f32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4f {
    let left = frame_width / material_size.x;
    let top = frame_width / material_size.y;
    let right = 1.0 - left;
    let bottom = 1.0 - top;

    if (mesh.uv.x < left || mesh.uv.x > right)  {
        if (dash_size > 0.0) {
            if (mesh.uv.y > top && mesh.uv.y < bottom) {
                let vdash = dash_size / material_size.y;
                let a = f32(u32(mesh.uv.y / vdash) % 2);
                return vec4f(material_color.rgb, a);
            }
        } else {
            return material_color;
        }
    }
    if (mesh.uv.y < top || mesh.uv.y > bottom)  {
        if (dash_size > 0.0) {
            if (mesh.uv.x > left && mesh.uv.x < right) {
                let hdash = dash_size / material_size.x;
                let a = f32(u32(mesh.uv.x / hdash) % 2);
                return vec4f(material_color.rgb, a);
            }
        } else {
            return material_color;
        }
    }

    return vec4f(material_color.rgb, 0.0);
}
