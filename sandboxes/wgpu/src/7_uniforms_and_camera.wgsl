// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
// This is where we specify the bind group that we want to use in the shader.
// Note that the group and binding needs to match what you set in rust land
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct InstanceInput {
    @location(2) translation: vec2<f32>,
    @location(3) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    in: VertexInput, instance: InstanceInput
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = in.tex_coords;
    // Multiplication order here is very important. Matrix multiplication is not transitive.
    // You can think of how the shape of the matrix will end up to give some hints.
    //
    // Here because we want to move the position, we want the view projection to be on the left
    out.clip_position = camera.view_proj * (vec4<f32>(in.position, 1.0) + vec4<f32>(instance.translation, 0.0, 0.0));
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
