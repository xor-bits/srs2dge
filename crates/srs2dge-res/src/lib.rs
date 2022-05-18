pub mod font {
    pub const FIRA: &[u8] = include_bytes!("../res/font/fira/font.ttf");
    pub const ROBOTO: &[u8] = include_bytes!("../res/font/roboto/font.ttf");
}

pub mod shader {
    pub const COLORED_2D: &str = include_str!("../res/shader/colored_2d.wgsl");
    pub const SDF: &str = include_str!("../res/shader/sdf.wgsl");
    pub const TEXT: &str = include_str!("../res/shader/text.wgsl");
    pub const TEXTURE_2D: &str = include_str!("../res/shader/texture_2d.wgsl");
}

pub mod texture {
    pub const EMPTY: &[u8] = include_bytes!("../res/texture/empty.png");
    pub const RUST: &[u8] = include_bytes!("../res/texture/rust.png");
    pub const SDF: &[u8] = include_bytes!("../res/texture/sdf.png");
    pub const SPRITE: &[u8] = include_bytes!("../res/texture/sprite.png");
}
