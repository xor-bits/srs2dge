struct FragmentInput {
	// @builtin(position) pos: vec4<f32>,
	// @location(0) col: vec4<f32>,
	// @location(1) uv: vec2<f32>,
	[[builtin(position)]] pos: vec4<f32>;
	[[location(0)]] col: vec4<f32>;
	[[location(1)]] uv: vec2<f32>;
};

// @group(0)
// @binding(1)
[[group(0), binding(1)]]
var t_texture: texture_2d<f32>;

// @group(0)
// @binding(2)
[[group(0), binding(2)]]
var s_texture: sampler;

fn process(base: vec2<i32>, kernel: mat3x3<f32>) -> vec3<f32> {
	var val = vec3<f32>(0.0);
	val = val + textureLoad(t_texture, base + vec2<i32>(-1, -1), 0).rgb * kernel[0][0];
	val = val + textureLoad(t_texture, base + vec2<i32>( 0, -1), 0).rgb * kernel[0][1];
	val = val + textureLoad(t_texture, base + vec2<i32>( 1, -1), 0).rgb * kernel[0][2];
	val = val + textureLoad(t_texture, base + vec2<i32>(-1,  0), 0).rgb * kernel[1][0];
	val = val + textureLoad(t_texture, base + vec2<i32>( 0,  0), 0).rgb * kernel[1][1];
	val = val + textureLoad(t_texture, base + vec2<i32>( 1,  0), 0).rgb * kernel[1][2];
	val = val + textureLoad(t_texture, base + vec2<i32>(-1,  1), 0).rgb * kernel[2][0];
	val = val + textureLoad(t_texture, base + vec2<i32>( 0,  1), 0).rgb * kernel[2][1];
	val = val + textureLoad(t_texture, base + vec2<i32>( 1,  1), 0).rgb * kernel[2][2];
	// for(var c: i32 = 0; c < 3; c++) { for(var r: i32 = 0; r < 3; r++) {
	//  val += textureLoad(t_texture, base + vec2<i32>(c - 1, r - 1), 0).rgb * kernel[c + 3 * r];
	// } }
	return val;
}

// @fragment
[[stage(fragment)]]
fn main(fin: FragmentInput) -> [[location(0)]] vec4<f32> { // @location(0)
	let dim = textureDimensions(t_texture);
	let pix = vec2<f32>(dim);
	let base = vec2<i32>(pix * fin.uv);

	// edge detection 1
	//var val = process(base, mat3x3<f32>(vec3<f32>(
	//	-1.0 , -1.0 , -1.0 ),vec3<f32>(
	//	-1.0 ,  8.0 , -1.0 ),vec3<f32>(
	//	-1.0 , -1.0 , -1.0 )));

	// edge detection 2
	//var val = process(base, mat3x3<f32>(vec3<f32>(
	//	 0.0 , -1.0 ,  0.0 ),vec3<f32>(
	//	-1.0 ,  4.0 , -1.0 ),vec3<f32>(
	//	 0.0 , -1.0 ,  0.0 )));

	// vertical edge detection
	//var val = process(base, mat3x3<f32>(vec3<f32>(
	//	 1.0 ,  0.0 , -1.0 ),vec3<f32>(
	//	 2.0 ,  0.0 , -2.0 ),vec3<f32>(
	//	 1.0 ,  0.0 , -1.0 )));

	// sharpen
	//var val = process(base, mat3x3<f32>(vec3<f32>(
	//	 0.0 , -1.0 ,  0.0 ),vec3<f32>(
	//	-1.0 ,  5.0 , -1.0 ),vec3<f32>(
	//	 0.0 , -1.0 ,  0.0 )));
	
	// blur 1
	//var val = 1.0 / 16.0 * process(base, mat3x3<f32>(vec3<f32>(
	//	 1.0 ,  2.0 ,  1.0 ),vec3<f32>(
	//	 2.0 ,  4.0 ,  2.0 ),vec3<f32>(
	//	 1.0 ,  2.0 ,  1.0 )));
	
	// blur 2
	//var val = 1.0 / 9.0 * process(base, mat3x3<f32>(vec3<f32>(
	//	 1.0 ,  1.0 ,  1.0 ),vec3<f32>(
	//	 1.0 ,  1.0 ,  1.0 ),vec3<f32>(
	//	 1.0 ,  1.0 ,  1.0 )));

	// blur 3
	let rad = 10;
	let mul = 1.0 / f32(rad * rad);
	var val = vec3<f32>(0.0);
	for(var c: i32 = -rad; c <= rad; c = c + 1) { for(var r: i32 = -rad; r <= rad; r = r + 1) {
	 	val = val + mul * textureLoad(t_texture, base + vec2<i32>(c, r), 0).rgb;
	} }

	// pass
	// var val = textureLoad(t_texture, base, 0).rgb;
	
	// inverse
	// var val = 1.0 - textureLoad(t_texture, base, 0).rgb;

	return vec4<f32>(val, 1.0);
}