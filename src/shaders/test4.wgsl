struct Input {
    @location(0) position: vec3<f32>,
    @location(1) colors: vec4<f32>
}
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec4<f32>
}
struct CameraUniform {
    data: mat4x4<f32>
}
@group(0) @binding(0)
var<uniform> camera: CameraUniform;
@vertex
fn vs_main(input: Input) -> Output {
    var output: Output;
    output.position = camera.data * vec4(input.position, 1.0);
    output.uv = input.colors;
    return output;
}
@fragment
fn fs_main(input: Output) -> @location(0) vec4<f32> {
    return input.uv;
}
