pub use srs2dge_core::*;

//

pub mod prelude;

#[cfg(feature = "ecs")]
pub mod ecs {
    pub use srs2dge_ecs::prelude::*;
}

#[cfg(feature = "gizmos")]
pub mod gizmos {
    pub use srs2dge_gizmos::prelude::*;
}

#[cfg(feature = "gui")]
pub mod gui {
    pub use srs2dge_gui::prelude::*;
}

#[cfg(feature = "gui-derive")]
pub mod gui_derive {
    pub use srs2dge_gui_derive::Widget;
}

#[cfg(feature = "presets")]
pub mod presets {
    pub use srs2dge_presets::*;
}

#[cfg(feature = "res")]
pub mod res {
    pub use srs2dge_res::*;
}

#[cfg(feature = "text")]
pub mod text {
    pub use srs2dge_text::prelude::*;
}
