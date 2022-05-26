struct VertexInput {
	// @location(0) pos: vec2<f32>,
	// @location(1) uv: vec2<f32>,
	// @location(2) col: vec4<f32>,
	[[location(0)]] pos: vec2<f32>;
	[[location(1)]] uv: vec2<f32>;
	[[location(2)]] col: vec4<f32>;
};

struct FragmentInput {
	// @builtin(position) pos: vec4<f32>,
	// @location(0) col: vec4<f32>,
	[[builtin(position)]] pos: vec4<f32>;
	[[location(0)]] col: vec4<f32>;
};

struct UniformInput {
	mvp: mat4x4<f32>;
};

// group(0)
// @binding(0)
[[group(0), binding(0)]]
var<uniform> ubo: UniformInput;

// @vertex
[[stage(vertex)]]
fn vs_main(vin: VertexInput) -> FragmentInput {
	var fin: FragmentInput;
	fin.pos = ubo.mvp * vec4<f32>(vin.pos, 0.0, 1.0);
	fin.col = vin.col;
	return fin;
}

// @fragment
[[stage(fragment)]]
fn fs_main(fin: FragmentInput) -> [[location(0)]] vec4<f32> { // @location(0)
	return fin.col;
}