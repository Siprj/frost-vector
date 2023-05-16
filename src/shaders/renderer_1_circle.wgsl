// Vertex shader

@group(0) @binding(0)
var<uniform> perspective: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv_coords: vec2<f32>,
}
struct InstanceInput {
    @location(2) position: vec2<f32>,
    @location(3) radius: f32,
//    @location(4) brush_width: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv_coords: vec2<f32>,
    @location(1) half_brush_width: f32,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    let model_matrix = mat4x4<f32>(
        vec4(1.0, 0.0, 0.0, 0.0), 
        vec4(0.0, 1.0, 0.0, 0.0), 
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(instance.position.x, instance.position.y, 0.0, 1.0)
    );
    let world_position = model_matrix * vec4<f32>(model.position.x * instance.radius, model.position.y * instance.radius, 0.5, 1.0);

    out.clip_position = perspective * world_position;
    out.uv_coords = model.uv_coords;
    // out.half_brush_width = (instance.brush_width/instance.radius)/2;
    out.half_brush_width = (10.0/instance.radius)/2.0;

    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let circle_sd: f32 = abs(length(in.uv_coords) - 1.0 + in.half_brush_width) - in.half_brush_width;
    // let circle_sd: f32 = length(in.uv_coords) - 1.0;

    if circle_sd > 0.0 {
        discard;
    }
    return vec4<f32>(1.0, 1.0, 0.0, 1.0);
}
