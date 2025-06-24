struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>
}

struct FragmentInput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>
}

struct Rect {
    position: vec2<f32>,
    size: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> screen_size: vec2<f32>;
@group(0) @binding(1)
var<uniform> rect: Rect;

@vertex
fn vertex_main(in: VertexInput) -> FragmentInput {
    var out:FragmentInput;
    let ndc_size = rect.size / screen_size;
    let rect_position = in.position * ndc_size + vec2<f32>(ndc_size.x, -ndc_size.y);
    let ndc_position = vec2<f32>(rect.position.x / screen_size.x * 2.0 - 1.0, 1.0 - (rect.position.y / screen_size.y) * 2.0);
    out.position = vec4<f32>(rect_position + ndc_position, 0.0, 1.0);
    out.uv = in.uv;
    return out;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var sampl: sampler;

@fragment
fn fragment_main(in:FragmentInput) -> @location(0) vec4<f32> {
    return textureSample(texture, sampl, in.uv);
}
