use std::f32::consts::PI;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use srs2dge::prelude::*;

//

const RES: u32 = 20;

//

#[derive(Debug, Clone, Copy)]
pub struct AsteroidMesh {
    pub seed: u64,
    pub lerp_transform: Transform2D,
}

//

impl Default for AsteroidMesh {
    fn default() -> Self {
        Self {
            seed: rand::thread_rng().gen(),
            lerp_transform: Default::default(),
        }
    }
}

impl Mesh<DefaultVertex> for AsteroidMesh {
    const PRIM: PrimitiveTopology = PrimitiveTopology::LineStrip;

    // TODO : #![feature(type_alias_impl_trait)]
    type VertexIter = Box<dyn Iterator<Item = DefaultVertex>>;
    type IndexIter = Box<dyn Iterator<Item = u32>>;

    fn vertices(&self) -> Self::VertexIter {
        let col = Color::WHITE;
        let translation = self.lerp_transform.translation;
        let radius = self.lerp_transform.scale;
        let rotation = self.lerp_transform.rotation;
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);

        Box::new(
            (0..RES)
                .map(move |i| i as f32 / RES as f32 * 2.0 * PI + rotation)
                .map(move |v| {
                    DefaultVertex::new(
                        translation
                            + rng.gen_range(0.6..1.0) * radius * Vec2::new(v.cos(), v.sin()),
                        col,
                        Vec2::ZERO,
                    )
                }),
        )
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        Box::new((0..RES).chain([0]).map(move |i| i + offset).chain([!0]))
    }

    fn index_step(&self) -> u32 {
        RES
    }
}
