use super::{
    prelude::{Rect, Target},
    Texture,
};
use image::RgbaImage;
use rapid_qoi::{Colors, Qoi};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

//

/// Texture cannot be serialized or deserialized
/// directly because it needs access to [`Target`]
/// to download/upload bytes from/to VRAM.
///
/// This [`SerializeableTexture`] works around this
/// by converting it to a
/// serializeable/deserializeable form first.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SerializeableTexture {
    pub image: RgbaQoiImage,
    pub label: Option<String>,
}

/// A helper struct to serialize an rgba image
/// compressed with QOI.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RgbaQoiImage {
    pub image: RgbaImage,
}

//

impl SerializeableTexture {
    /// Conststruct a new [`SerializeableTexture`]
    /// headlessly.
    pub fn new(image: RgbaImage, label: Option<String>) -> Self {
        Self {
            image: RgbaQoiImage { image },
            label,
        }
    }

    /// Split this texture to its image and label parts.
    pub fn split(self) -> (RgbaImage, Option<String>) {
        (self.image.image, self.label)
    }

    /// Download this texture from VRAM to RAM
    /// (GPU to CPU) handled by [`Self`].
    pub async fn download<const USAGE: u32>(
        texture: &Texture<USAGE>,
        target: &Target,
        label: Option<String>,
    ) -> Self {
        texture.download(target, label).await
    }

    /// Upload this texture from RAM to VRAM
    /// (CPU to GPU) handled by [`Texture`].
    pub fn upload<const USAGE: u32>(&self, target: &Target) -> Texture<USAGE> {
        Texture::new_rgba_with(target, &self.image.image, self.label.as_deref())
    }

    pub fn get_dim(&self) -> Rect {
        self.image.image.dimensions().into()
    }
}

impl<const USAGE: u32> Texture<USAGE> {
    /// Download this texture from VRAM to RAM
    /// (GPU to CPU) handled by [`Self`].
    pub async fn download(&self, target: &Target, label: Option<String>) -> SerializeableTexture {
        let image = RgbaQoiImage {
            image: self.read(target).await.into_rgba8(),
        };
        SerializeableTexture { image, label }
    }

    /// Upload this texture from RAM to VRAM
    /// (CPU to GPU) handled by [`Texture`].
    pub fn upload(from: &SerializeableTexture, target: &Target) -> Self {
        SerializeableTexture::upload(from, target)
    }
}

impl Serialize for RgbaQoiImage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let qoi = Qoi {
            width: self.image.width(),
            height: self.image.height(),
            colors: Colors::Rgba,
        };

        let image = qoi
            .encode_alloc(self.image.as_raw())
            .map_err(serde::ser::Error::custom)?;

        image.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RgbaQoiImage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let image = Vec::<u8>::deserialize(deserializer)?;
        let (qoi, image) = Qoi::decode_alloc(&image).map_err(serde::de::Error::custom)?;

        Ok(Self {
            image: RgbaImage::from_raw(qoi.width, qoi.height, image).unwrap(),
        })
    }
}
