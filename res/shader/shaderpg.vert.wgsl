struct FragmentInput {
	@builtin(position) pos: vec4<f32>,
	@location(0) uv: vec2<f32>,
};

@vertex
fn main(@builtin(vertex_index) idx: u32) -> FragmentInput {
	let u = f32(idx % 2u);
	let v = f32(idx / 2u);
	let x = u * 2.0 - 1.0;
	let y = v * 2.0 - 1.0;

	var fin: FragmentInput;
	fin.pos = vec4<f32>(x, y, 0.0, 1.0);
	fin.uv = vec2<f32>(u, v);

	return fin;
}