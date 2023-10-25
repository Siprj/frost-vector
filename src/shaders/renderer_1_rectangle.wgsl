// Vertex shader

@group(0) @binding(0)
var<uniform> perspective: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv_coords: vec2<f32>,
}

struct InstanceInput {
    @location(2) position: vec2<f32>,
    @location(3) size: vec2<f32>,
    @location(4) brush_size: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) rect_pos: vec2<f32>,
    @location(1) brush_size: f32,
    @location(2) rectangle_half_size: vec2<f32>,
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
    let world_position = model_matrix * vec4<f32>(model.position.x * instance.size.x, model.position.y * instance.size.y, 0.5, 1.0);

    out.clip_position = perspective * world_position;
    out.rect_pos = vec2<f32>((instance.size.x / 2.0) * model.uv_coords.x, (instance.size.y / 2.0) * model.uv_coords.y);
    out.brush_size = instance.brush_size * 0.5;
    out.rectangle_half_size = instance.size / 2.0;

    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let v = abs(in.rect_pos) - in.rectangle_half_size;
    let inner_sd = min(max(v.x, v.y), 0.0); 

    if inner_sd < (-in.brush_size){
        discard;
    }
    return vec4<f32>(1.0, 1.0, 0.0, 1.0);
}
