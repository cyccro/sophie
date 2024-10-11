struct Input {
    @location(0) position: vec3<f32>,
    @location(1) txt_coords: vec2<f32>
}
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>
}

@vertex
fn vs_main(input: Input) -> Output {
    var output: Output;
    output.position = vec4(input.position, 1.0);
    output.uv = input.txt_coords;
    return output;
}
@group(0) @binding(0)
var diffuse: texture_2d<f32>;
@group(0) @binding(1)
var sdiff: sampler;
@fragment
fn fs_main(input: Output) -> @location(0) vec4<f32> {
    return textureSample(diffuse, sdiff, input.uv);
}
