// Vertex shader

@group(0) @binding(0)
var<uniform> perspective: mat4x4<f32>;

struct InstanceInput {
    @location(1) position: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) brush_size: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

fn position_from_index(vertex_index: u32, instance: InstanceInput) -> vec2<f32> {
    switch vertex_index {
        case 0u: {
            let x = instance.position.x - (instance.size.x/2.0);
            let y = instance.position.y - (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }
        case 1u: {
            let x = instance.position.x - (instance.size.x/2.0);
            let y = instance.position.y + (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }
        case 2u: {
            let x = instance.position.x - (instance.size.x/2.0) + instance.brush_size;
            let y = instance.position.y + (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }

        case 3u: {
            let x = instance.position.x - (instance.size.x/2.0);
            let y = instance.position.y - (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }
        case 4u: {
            let x = instance.position.x - (instance.size.x/2.0) + instance.brush_size;
            let y = instance.position.y + (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }
        case 5u: {
            let x = instance.position.x - (instance.size.x/2.0) + instance.brush_size;
            let y = instance.position.y - (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }

        case 6u: {
            let x = instance.position.x - (instance.size.x/2.0) + instance.brush_size;
            let y = instance.position.y - (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }
        case 7u: {
            let x = instance.position.x - (instance.size.x/2.0) + instance.brush_size;
            let y = instance.position.y - (instance.size.y/2.0) + instance.brush_size;
            return vec2<f32>(x,y);
        }
        case 8u: {
            let x = instance.position.x + (instance.size.x/2.0) - instance.brush_size;
            let y = instance.position.y - (instance.size.y/2.0) + instance.brush_size;
            return vec2<f32>(x,y);
        }

        case 9u: {
            let x = instance.position.x - (instance.size.x/2.0) + instance.brush_size;
            let y = instance.position.y - (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }
        case 10u: {
            let x = instance.position.x + (instance.size.x/2.0) - instance.brush_size;
            let y = instance.position.y - (instance.size.y/2.0) + instance.brush_size;
            return vec2<f32>(x,y);
        }
        case 11u: {
            let x = instance.position.x + (instance.size.x/2.0) - instance.brush_size;
            let y = instance.position.y - (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }

        case 12u: {
            let x = instance.position.x + (instance.size.x/2.0) - instance.brush_size;
            let y = instance.position.y - (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }
        case 13u: {
            let x = instance.position.x + (instance.size.x/2.0);
            let y = instance.position.y + (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }
        case 14u: {
            let x = instance.position.x + (instance.size.x/2.0);
            let y = instance.position.y - (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }

        case 15u: {
            let x = instance.position.x + (instance.size.x/2.0) - instance.brush_size;
            let y = instance.position.y - (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }
        case 16u: {
            let x = instance.position.x + (instance.size.x/2.0) - instance.brush_size;
            let y = instance.position.y + (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }
        case 17u: {
            let x = instance.position.x + (instance.size.x/2.0);
            let y = instance.position.y + (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }

        case 18u: {
            let x = instance.position.x - (instance.size.x/2.0) + instance.brush_size;
            let y = instance.position.y + (instance.size.y/2.0) - instance.brush_size;
            return vec2<f32>(x,y);
        }
        case 19u: {
            let x = instance.position.x + (instance.size.x/2.0) - instance.brush_size;
            let y = instance.position.y + (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }
        case 20u: {
            let x = instance.position.x + (instance.size.x/2.0) - instance.brush_size;
            let y = instance.position.y + (instance.size.y/2.0) - instance.brush_size;
            return vec2<f32>(x,y);
        }

        case 21u: {
            let x = instance.position.x - (instance.size.x/2.0) + instance.brush_size;
            let y = instance.position.y + (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }
        case 22u: {
            let x = instance.position.x + (instance.size.x/2.0) - instance.brush_size;
            let y = instance.position.y + (instance.size.y/2.0);
            return vec2<f32>(x,y);
        }
        case 23u: {
            let x = instance.position.x - (instance.size.x/2.0) + instance.brush_size;
            let y = instance.position.y + (instance.size.y/2.0) - instance.brush_size;
            return vec2<f32>(x,y);
        }
        default: {
          return vec2<f32>(0.0,0.0);
        }
    }
}

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    let p = position_from_index(vertex_index, instance);
    out.clip_position = perspective * vec4<f32>(p.x, p.y, 0.5, 1.0);
    // out.clip_position = vec4(p.x, p.y, 0.5, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 0.0, 1.0);
}
