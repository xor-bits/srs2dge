use glam::Vec2;
use wgpu::PresentMode;

//

pub trait ForceAspectRatio {
    fn force_ratio_with_x(self, ratio: f32) -> Self;
    fn force_ratio_with_y(self, ratio: f32) -> Self;
}

//

impl ForceAspectRatio for Vec2 {
    fn force_ratio_with_x(mut self, ratio: f32) -> Self {
        self.x = self.y * ratio;
        self
    }

    fn force_ratio_with_y(mut self, ratio: f32) -> Self {
        self.y = self.x / ratio;
        self
    }
}

//

pub fn present_mode_from_env() -> Option<PresentMode> {
    std::env::var("PRESENT_MODE")
        .as_deref()
        .map(str::to_lowercase)
        .ok()
        .as_deref()
        .and_then(present_mode_from_str)
}

pub fn present_mode_from_str(string: &str) -> Option<PresentMode> {
    match string.to_lowercase().trim() {
        "mailbox" | "mail" | "sync" | "mb" | "m" | "s" => Some(PresentMode::Mailbox),
        "fifo" | "f" => Some(PresentMode::Fifo),
        "immediate" | "nosync" | "im" | "i" => Some(PresentMode::Immediate),
        b => {
            log::warn!("unknown present mode string '{}'", b);
            None
        }
    }
}

//

#[macro_export]
macro_rules! unwrap_or_return {
    ($e:expr) => {
        match $e {
            Some(__some) => __some,
            None => return,
        }
    };
}
