struct VertexInput {
    @location(0) position: vec3<f32>,
    // note(jglass): Added this to mirror the stuff in Rust
    @location(1) tex_coords: vec2<f32>,
}

struct InstanceInput {
    @location(2) translation: vec2<f32>,
    @location(3) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    // note(jglass): Need to pass this through to the fragment shader
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    in: VertexInput, instance: InstanceInput
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = in.tex_coords;
    out.clip_position = vec4<f32>(in.position, 1.0) + vec4<f32>(instance.translation, 0.0, 0.0);
    return out;
}

// These variables are uniforms. The bind groups are a collection of uniforms
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
