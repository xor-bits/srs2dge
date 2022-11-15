use tracing::span::EnteredSpan;

//

pub struct ComputePass<'e> {
    _inner: wgpu::ComputePass<'e>,

    _span: EnteredSpan,
}

//

impl<'e> ComputePass<'e> {
    pub(crate) fn new(_inner: wgpu::ComputePass<'e>, _span: EnteredSpan) -> Self {
        Self { _inner, _span }
    }
}
