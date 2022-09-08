use super::{LayoutStyle, Style, WidgetStyle};

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
            widget: self.widget.merge(other.widget),
            layout: self.layout.merge(other.layout),
        }
    }
}

impl MergeStyles for LayoutStyle {
    fn merge(self, other: Self) -> Self {
        Self {
            display: self.display.merge(other.display),
            position_type: self.position_type.merge(other.position_type),
            flex_direction: self.flex_direction.merge(other.flex_direction),
            flex_wrap: self.flex_wrap.merge(other.flex_wrap),
            align_items: self.align_items.merge(other.align_items),
            align_self: self.align_self.merge(other.align_self),
            align_content: self.align_content.merge(other.align_content),
            justify_content: self.justify_content.merge(other.justify_content),
            position: self.position.merge(other.position),
            margin: self.margin.merge(other.margin),
            padding: self.padding.merge(other.padding),
            border: self.border.merge(other.border),
            flex_grow: self.flex_grow.merge(other.flex_grow),
            flex_shrink: self.flex_shrink.merge(other.flex_shrink),
            flex_basis: self.flex_basis.merge(other.flex_basis),
            size: self.size.merge(other.size),
            min_size: self.min_size.merge(other.min_size),
            max_size: self.max_size.merge(other.max_size),
            aspect_ratio: self.aspect_ratio.merge(other.aspect_ratio),
        }
    }
}

impl MergeStyles for WidgetStyle {
    fn merge(self, other: Self) -> Self {
        Self {
            color: self.color.merge(other.color),
            background_color: self.background_color.merge(other.background_color),
            texture: self.texture.merge(other.texture),
        }
    }
}

impl<T> MergeStyles for Option<T> {
    fn merge(self, other: Self) -> Self {
        other.or(self)
    }
}
