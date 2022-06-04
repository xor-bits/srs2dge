use super::prelude::TextChars;

//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextBoundingBox {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextBoundingBoxTypo {
    /// left to right for [`TextDirection::Right`]
    /// top to bottom [`TextDirection::Down`]
    pub width: u32,

    /// top to bottom for [`TextDirection::Down`]
    /// left to right [`TextDirection::Right`]
    pub height: u32,
}

//

impl<'s> TextChars<'s> {
    /// not a typographic bounding box
    ///
    /// bounding box for the generated quads
    pub fn bounding_box(mut self) -> TextBoundingBox {
        if let Some(first) = self.next() {
            let mut min_x = first.x;
            let mut min_y = first.y;
            let mut max_x = first.x + first.width as i32;
            let mut max_y = first.y + first.height as i32;

            for c in self {
                min_x = min_x.min(c.x);
                min_y = min_y.min(c.y);
                max_x = max_x.max(c.x + c.width as i32);
                max_y = max_y.max(c.y + c.height as i32);
            }

            let width = max_x.abs_diff(min_x);
            let height = max_y.abs_diff(min_y);

            TextBoundingBox {
                x: min_x,
                y: min_y,
                width,
                height,
            }
        } else {
            TextBoundingBox {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            }
        }
    }

    pub fn bounding_box_typo(mut self) -> TextBoundingBoxTypo {
        // TODO: optimize for monospaced fonts

        let xo = self.x_origin;
        let yo = self.y_origin;
        let mut width = 0;
        let mut height = 0;

        while self.next().is_some() {
            width = width.max(self.x_origin.abs_diff(xo));
            height = height.max(self.y_origin.abs_diff(yo));
        }

        self.new_line(self.init());

        width = width.max(self.x_origin.abs_diff(xo));
        height = height.max(self.y_origin.abs_diff(yo));

        TextBoundingBoxTypo { width, height }
    }
}
