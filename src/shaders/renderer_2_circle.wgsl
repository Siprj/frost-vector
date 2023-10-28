// Vertex shader

@group(0) @binding(0)
var<uniform> perspective: mat4x4<f32>;

struct InstanceInput {
    @location(1) position: vec2<f32>,
    @location(2) radius: f32,
    @location(3) brush_size: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

fn position_from_index(vertex_index: u32) -> vec2<f32> {
    switch vertex_index {
        case 0u: {
            let x = 0.0;
            let y = 0.0;
            return vec2<f32>(x,y);
        }
        case 1u: {
            let x = 0.0;
            let y = 400.0;
            //let y = 1.0;
            return vec2<f32>(x,y);
        }
        case 2u: {
            let x = 400.0;
            let y = 400.0;
            return vec2<f32>(x,y);
        }

        case 3u: {
            let x = 400.0;
            let y = 0.0;
            return vec2<f32>(x,y);
        }
        default: {
          return vec2<f32>(500.0,500.0);
        }
    }
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;

    let p = position_from_index(vertex_index);
    out.clip_position = perspective * vec4<f32>(p.x, p.y, 0.5, 1.0);



    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, -0.1, 1.0);
}
