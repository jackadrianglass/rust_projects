// Vertex shader

struct SimpleUniform {
    translation: vec2<f32>,
    color: vec3<f32>,
};

// note(jglass)
// This is describing where to get the data from the uniform layout
// - group refers to the index that is before the render call to the uniform buffer
// - binding refers to the number in the pipeline layout that you have defined. If you're
//   making multiple uniform buffers, then the binding would be different
//
// Basically this allows you to bind different uniform buffers to different pipeline
// layouts that you have thought of
@group(0) @binding(0)
var<uniform> uniforms: SimpleUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    in: VertexInput
) -> VertexOutput {
    var out: VertexOutput;

    // note(jglass)
    // 
    // You can refer to the uniforms in the shader program so long as you specify that the shader
    // step has access to the uniform. This is defined in the bind group layout descriptor
    //
    // In this example, you cannot access the uniform from the fragment shader since the visibility
    // is only set to `vertex`
    out.clip_position = vec4<f32>(in.position, 1.0) + vec4<f32>(uniforms.translation, 0.0, 0.0);
    out.color = uniforms.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
