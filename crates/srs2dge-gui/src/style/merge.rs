use super::{Style, StyleRef};

//

pub trait MergeStyles: Sized {
    /// overwrites values of self with
    /// values from other if they are `Some(..)`
    fn merge(self, other: Self) -> Self;
}

//

impl MergeStyles for Style {
    fn merge(self, other: Self) -> Self {
        Self {
            color: other.color.merge(self.color),
            texture: other.texture.merge(self.texture),
            size: other.size.merge(self.size),
            offset: other.offset.merge(self.offset),
        }
    }
}

impl<'a> MergeStyles for StyleRef<'a> {
    fn merge(self, other: Self) -> Self {
        Self {
            color: other.color.merge(self.color),
            texture: other.texture.merge(self.texture),
            size: other.size.merge(self.size),
            offset: other.offset.merge(self.offset),
        }
    }
}

impl<T> MergeStyles for Option<T> {
    fn merge(self, other: Self) -> Self {
        other.or(self)
    }
}
