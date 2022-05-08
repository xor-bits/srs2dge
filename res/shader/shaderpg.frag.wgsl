struct FragmentInput {
	@builtin(position) pos: vec4<f32>,
	@location(0) uv: vec2<f32>,
};

struct UniformInput {
	aspect: f32,
	time: f32
}

@group(0)
@binding(0)
var<uniform> ubo: UniformInput;

@fragment
fn main(fin: FragmentInput) -> @location(0) vec4<f32> {
	return vec4<f32>(fin.uv, sin(ubo.time * 5.0) * 0.5 + 0.5, 1.0);
}