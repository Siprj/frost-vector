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
    @location(0) uv_coords: vec2<f32>,
    @location(1) brush_size: f32,
    @location(2) brush_scale_compensation: f32,
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
    out.uv_coords = model.uv_coords;
    out.brush_size = instance.brush_size / instance.size.x;
    out.brush_scale_compensation = instance.size.x / instance.size.y;

    return out;
}

// Fragment shader

fn rectangle_sdf(uv_pos: vec2<f32>, ) -> f32 {
    let componentWiseEdgeDistance = abs(uv_pos) - 1.0;
    let outsideDistance = length(max(componentWiseEdgeDistance, vec2(0.0, 0.0)));
    let insideDistance = min(max(componentWiseEdgeDistance.x, componentWiseEdgeDistance.y), 0.0);
    return outsideDistance + insideDistance;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let compensated_uv = vec2(in.uv_coords.x, abs(in.uv_coords.y) + (in.brush_size * in.brush_scale_compensation));
    let rectangle_sd: f32 = rectangle_sdf(compensated_uv);

    if rectangle_sd < (0.0 - in.brush_size){
        discard;
    }
    return vec4<f32>(1.0, 1.0, 0.0, 1.0);
}