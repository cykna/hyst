struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) instance_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct Rect {
    position: vec2<f32>,
    size: vec2<f32>,
};

struct ScreenSize {
    size: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> screen_size: ScreenSize;

@group(0) @binding(1)
var<uniform> rect: Rect;

@vertex
fn vs_main(
    model: VertexInput,
    @builtin(vertex_index) vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    // Calcula posição do vértice no espaço de tela
    let vertex_pos = model.position * rect.size / 2.0 + rect.position;
    let normalized_pos = vertex_pos / screen_size.size * 2.0 - 1.0;
    out.clip_position = vec4<f32>(normalized_pos.x, -normalized_pos.y, 0.0, 1.0);
    out.color = model.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
