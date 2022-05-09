use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use wgpu::{
    Adapter, Device, Instance, PresentMode, SurfaceConfiguration, SurfaceError, SurfaceTexture,
    TextureFormat, TextureUsages,
};
use winit::window::Window;

//

pub struct ISurface {
    instance: Arc<Instance>,
    surface: wgpu::Surface,
    window: Arc<Window>,
}

pub struct Surface {
    device: Arc<Device>,
    surface: ISurface,
    format: TextureFormat,

    width: u32,
    height: u32,
}

//

impl ISurface {
    pub fn new(window: Arc<Window>, instance: Arc<Instance>) -> Self {
        // SAFETY: the window is held in an `Arc`.
        // It is dropped before window is dropped,
        // because it will be the first elem in this
        // struct.
        //
        // `create_surface` requires "Raw Window Handle
        // must be a valid object to create a surface
        // upon and must remain valid for the lifetime
        // of the returned surface."
        let surface = unsafe { instance.create_surface(window.as_ref()) };

        Self {
            instance,
            surface,
            window,
        }
    }

    pub fn complete(self, adapter: &Adapter, device: Arc<Device>) -> Surface {
        let surface = self;
        let format = surface
            .surface
            .get_preferred_format(adapter)
            .expect("Surface is not incompatible");

        let mut surface = Surface {
            device,
            surface,
            format,

            width: 0, // properly configured in just a bit
            height: 0,
        };
        surface.configure();
        surface
    }

    pub fn get_window(&self) -> Arc<Window> {
        self.window.clone()
    }
}

impl Surface {
    pub fn configure(&mut self) {
        let window = self.surface.window.as_ref();
        let size = window.inner_size();
        let (width, height) = (size.width, size.height);
        let format = self.format;

        self.width = width;
        self.height = height;
        self.surface.surface.configure(
            &self.device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format,
                width,
                height,
                present_mode: PresentMode::Mailbox,
            },
        );
    }

    pub fn recreate(&mut self) {
        let window = self.surface.window.clone();
        let instance = self.surface.instance.clone();
        self.surface = ISurface::new(window, instance);
    }

    pub fn acquire(&mut self) -> SurfaceTexture {
        loop {
            match self.surface.get_current_texture() {
                // got texture
                Ok(
                    texture @ SurfaceTexture {
                        suboptimal: false, ..
                    },
                ) => return texture,

                // the only unrecoverable error: out of memory
                Err(SurfaceError::OutOfMemory) => panic!("Out of memory"),

                // retry
                Err(SurfaceError::Timeout) => {
                    log::debug!("Timeout");
                }

                // recreate the surface
                Err(SurfaceError::Lost) => {
                    log::debug!("Lost");
                    self.recreate();
                }

                // recreate the swapchain
                x @ (Ok(SurfaceTexture {
                    suboptimal: true, ..
                })
                | Err(SurfaceError::Outdated)) => {
                    log::debug!("Outdated | Suboptimal");
                    drop(x);
                    self.configure();
                }
            }
        }
    }

    pub fn format(&self) -> TextureFormat {
        self.format
    }

    pub fn get_window(&self) -> Arc<Window> {
        self.surface.get_window()
    }

    pub fn get_dim(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Deref for ISurface {
    type Target = wgpu::Surface;

    fn deref(&self) -> &Self::Target {
        &self.surface
    }
}

impl DerefMut for ISurface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.surface
    }
}

impl Deref for Surface {
    type Target = wgpu::Surface;

    fn deref(&self) -> &Self::Target {
        &self.surface
    }
}

impl DerefMut for Surface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.surface
    }
}
