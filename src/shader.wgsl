// Vertex shader

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] col: vec4<f32>;
};

[[stage(vertex)]]
fn main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
	let c = i32(in_vertex_index);
	out.col = vec4<f32>(0.0, 0.0, 0.0, 1.0);
	if (c == 0)
	{
		out.col = vec4<f32>(1.0, 0.0, 0.0, 1.0);
	}
	if (c == 1)
	{
		out.col = vec4<f32>(0.0, 1.0, 0.0, 1.0);
	}
	if (c == 2)
	{
		out.col = vec4<f32>(0.0, 0.0, 1.0, 1.0);
	}
    return out;
}

// Fragment shader

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.col;
}