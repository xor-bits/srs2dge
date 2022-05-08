use bytemuck::Pod;
use wgpu::IndexFormat;

//

pub type DefaultIndex = u32;

//

pub trait Index: Pod {
    const FORMAT: IndexFormat;
}

//

impl Index for u16 {
    const FORMAT: IndexFormat = IndexFormat::Uint16;
}

impl Index for u32 {
    const FORMAT: IndexFormat = IndexFormat::Uint32;
}

impl Index for () {
    const FORMAT: IndexFormat = IndexFormat::Uint16;
}
