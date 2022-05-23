pub use srs2dge_core::*;

//

pub mod prelude;

#[cfg(feature = "presets")]
pub mod presets {
    pub use srs2dge_presets::*;
}

#[cfg(feature = "gizmos")]
pub mod gizmos {
    pub use srs2dge_gizmos::prelude::*;
}

#[cfg(feature = "res")]
pub mod res {
    pub use srs2dge_res::*;
}

#[cfg(feature = "ecs")]
pub mod ecs {
    pub use srs2dge_ecs::prelude::*;
}
