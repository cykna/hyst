struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) position: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) color: vec4<f32>
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> screen_size: vec2<f32>;

@vertex
fn vertex_main(vx_in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let size = vx_in.size / screen_size;
    let scaled = vx_in.pos * size + vec2<f32>(size.x, -size.y);
    let pos = vec2<f32>(
        vx_in.position.x / screen_size.x * 2.0 - 1.0,
        1.0 - (vx_in.position.y / screen_size.y) * 2.0
    );
    
    out.position = vec4(scaled + pos, 0.0, 1.0);
    out.color = vx_in.color;
    return out;
}

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
