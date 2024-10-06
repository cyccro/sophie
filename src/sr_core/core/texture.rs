use std::{io::Read, path::Path};

use image::{GenericImageView, ImageError};

pub enum TextureCreationError {
    InvalidBytes(ImageError),
    InvalidPath(std::io::Error),
}

pub enum TextureError {
    Creation(TextureCreationError),
}

pub struct Texture {
    width: u32,
    height: u32,
}

impl Texture {
    pub fn from_file(path: &Path) -> Result<Self, TextureCreationError> {
        Self::new(
            std::fs::read(path)
                .map_err(|e| TextureCreationError::InvalidPath(e))?
                .as_slice(),
        )
    }
    pub async fn from_file_async(path: &Path) -> Result<Self, TextureCreationError> {
        Self::new(
            tokio::fs::read(path)
                .await
                .map_err(|e| TextureCreationError::InvalidPath(e))?
                .as_slice(),
        )
    }
    pub fn new(bytes: &[u8]) -> Result<Self, TextureCreationError> {
        let img =
            image::load_from_memory(bytes).map_err(|e| TextureCreationError::InvalidBytes(e))?;
        //let rgba = img.to_rgb8();
        let dimensions = img.dimensions();
        Ok(Self {
            width: dimensions.0,
            height: dimensions.1,
        })
    }
}
