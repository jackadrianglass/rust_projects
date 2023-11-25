// Vertex shader
struct VertexOutput {
	// This doesn't have the same value between the vertex shader and
	// the fragment shader
	// In the vertex shader, it's the vertex position in space
	// In the fragment shader, it's the interpolated screen space coordinate
	// which means that it's not likely to be between 0 and 1
	@builtin(position) clip_position: vec4<f32>,
	@location(0) position: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
// technically don't need a struct in this example but we're
// going to add more fields into the vertex output so might as
// well get used to it
) -> VertexOutput {
	// `var` variables can be modified but must specify their type
    var out: VertexOutput;

	// `let` variables can have their types inferred
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;

	// 0
	// (0.5, -0.5)
	// 1
	// (0, 0.5)
	// 2
	// (-0.5, -0.5)

	out.position = vec2<f32>(x, y);
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.position, 0.5, 1.0);
}
