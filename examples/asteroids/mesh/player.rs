use std::array::IntoIter;

use srs2dge::prelude::*;

//

#[derive(Debug, Clone, Copy, Default)]
pub struct PlayerMesh {
    pub lerp_transform: Transform2D,
}

//

impl Mesh<DefaultVertex> for PlayerMesh {
    const PRIM: PrimitiveTopology = PrimitiveTopology::LineStrip;

    // TODO : #![feature(type_alias_impl_trait)]
    type VertexIter = IntoIter<DefaultVertex, 4>;
    type IndexIter = IntoIter<DefaultIndex, 6>;

    fn vertices(&self) -> Self::VertexIter {
        let mat = Mat2::from_scale_angle(self.lerp_transform.scale, self.lerp_transform.rotation);
        let p0 = mat * Vec2::new(0.0, 1.0) + self.lerp_transform.translation;
        let p1 = mat * Vec2::new(-0.9, -1.0) + self.lerp_transform.translation;
        let p2 = mat * Vec2::new(0.0, -0.7) + self.lerp_transform.translation;
        let p3 = mat * Vec2::new(0.9, -1.0) + self.lerp_transform.translation;

        IntoIterator::into_iter([
            DefaultVertex::new(p0, Color::WHITE, Vec2::ZERO),
            DefaultVertex::new(p1, Color::WHITE, Vec2::ZERO),
            DefaultVertex::new(p2, Color::WHITE, Vec2::ZERO),
            DefaultVertex::new(p3, Color::WHITE, Vec2::ZERO),
        ])
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        IntoIterator::into_iter([offset, offset + 1, offset + 2, offset + 3, offset, !0])
    }

    fn index_step(&self) -> u32 {
        4
    }
}
