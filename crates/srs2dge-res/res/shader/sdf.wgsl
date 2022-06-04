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
	// @location(1) uv: vec2<f32>,
	[[builtin(position)]] pos: vec4<f32>;
	[[location(0)]] col: vec4<f32>;
	[[location(1)]] uv: vec2<f32>;
};

struct UniformInput {
	mvp: mat4x4<f32>;
	weight: f32;
	anti_alias: f32;
	border: f32;
};

// @group(0)
// @binding(0)
[[group(0), binding(0)]]
var<uniform> ubo: UniformInput;

// @group(0)
// @binding(1)
[[group(0), binding(1)]]
var t_texture: texture_2d<f32>;

// @group(0)
// @binding(2)
[[group(0), binding(2)]]
var s_texture: sampler;

// @vertex
[[stage(vertex)]]
fn vs_main(vin: VertexInput) -> FragmentInput {
	var fin: FragmentInput;
	fin.pos = ubo.mvp * vec4<f32>(vin.pos, 0.0, 1.0);
	fin.col = vin.col;
	fin.uv = vin.uv;
	return fin;
}

// smoothstep for old wgsl
// https://en.wikipedia.org/wiki/Smoothstep
fn smoothstep_2(edge0: f32, edge1: f32, x: f32) -> f32
{
   if (x < edge0) {
      return 0.0;
   }

   if (x >= edge1) {
      return 1.0;
   }

   // Scale/bias into [0..1] range
   let x = (x - edge0) / (edge1 - edge0);

   return x * x * (3.0 - 2.0 * x);
}

// @fragment
[[stage(fragment)]]
fn fs_main(fin: FragmentInput) -> [[location(0)]] vec4<f32> { // @location(0)
	let val = textureSample(t_texture, s_texture, fin.uv).x * 2.0 - 1.0;

	// smooth edges
	let aa = ubo.anti_alias;
	// border begin weight
	let bbw = -ubo.weight - ubo.border;
	// border end weigth
	let bew = -ubo.weight;

	// border
	let alpha = smoothstep_2(bbw, bbw + aa, val);

	// outline
	let col = vec3<f32>(smoothstep_2(bew, bew + aa, val));
	return fin.col * vec4<f32>(col, alpha);
}

