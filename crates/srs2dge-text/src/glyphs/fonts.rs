use fontsdf::Font;

//

#[derive(Debug, Clone)]
pub struct Fonts {
    fonts: Vec<Font>,
    fallback: Font,
}

//

impl Fonts {
    pub fn new(fallback: Font) -> Self {
        Self {
            fonts: Default::default(),
            fallback,
        }
    }

    pub fn new_bytes(fallback_bytes: &[u8]) -> Result<Self, &'static str> {
        Ok(Self {
            fonts: Default::default(),
            fallback: Font::from_bytes(fallback_bytes)?,
        })
    }

    /// Add a font to font map
    ///
    /// returns a handle to it
    ///
    /// this handle is used to format text
    pub fn add_font(&mut self, font: Font) -> usize {
        let id = self.fonts.len() + 1;
        self.fonts.push(font);
        id
    }

    /// Add a font to font map
    ///
    /// returns a handle to it
    ///
    /// this handle is used to format text
    pub fn add_font_bytes(&mut self, font: &[u8]) -> Result<usize, &'static str> {
        Ok(self.add_font(Font::from_bytes(font)?))
    }

    /// Get a font with its handle
    ///
    /// 0 = fallback
    /// others = added fonts
    pub fn get_font(&self, font: usize) -> &'_ Font {
        font.checked_sub(1)
            .and_then(|font| self.fonts.get(font))
            .unwrap_or(&self.fallback)
    }
}
