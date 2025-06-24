struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct Rect {
    position: vec2<f32>,
    size: vec2<f32>
}

@group(0) @binding(0)
var<uniform> screen_size: vec2<f32>;
@group(0) @binding(1)
var<uniform> rect: Rect;

@vertex
fn vertex_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let size = rect.size / screen_size;
    let scaled = in.pos * size + vec2<f32>(size.x, -size.y);
    let pos = vec2<f32>(
        rect.position.x / screen_size.x * 2.0 - 1.0,
        1.0 - (rect.position.y / screen_size.y) * 2.0
    );
    
    out.position = vec4(scaled + pos, 0.0, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
