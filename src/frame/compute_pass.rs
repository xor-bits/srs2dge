//

pub struct ComputePass<'e> {
    _inner: wgpu::ComputePass<'e>,
}

//

impl<'e> ComputePass<'e> {
    pub(crate) fn new(_inner: wgpu::ComputePass<'e>) -> Self {
        Self { _inner }
    }
}
