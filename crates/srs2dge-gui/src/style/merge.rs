use super::{Mergeable, Ref, Style};

//

pub trait MergeStyles: Sized {
    /// overwrites values of self with
    /// values from other if they are `Some(..)`
    fn merge(self, other: Self) -> Self;
}

//

impl MergeStyles for Style<Mergeable> {
    fn merge(self, other: Self) -> Self {
        Self {
            color: other.color.merge(self.color),
            texture: other.texture.merge(self.texture),
            size: other.size.merge(self.size),
            offset: other.offset.merge(self.offset),
            text_align: other.text_align.merge(self.text_align),
        }
    }
}

impl<'a> MergeStyles for Style<Ref<'a>> {
    fn merge(self, other: Self) -> Self {
        Self {
            color: other.color.merge(self.color),
            texture: other.texture.merge(self.texture),
            size: other.size.merge(self.size),
            offset: other.offset.merge(self.offset),
            text_align: other.text_align.merge(self.text_align),
        }
    }
}

impl<T> MergeStyles for Option<T> {
    fn merge(self, other: Self) -> Self {
        other.or(self)
    }
}
