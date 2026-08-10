use crate::*;

use image::{DynamicImage, ImageFormat};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    Normal,
    Thumbnail {
        nwidth: u32,
        nheight: u32,
        quality: u32,
        lossless: bool,
    },
}

impl ImageType {
    pub fn name(&self, name: &String) -> String {
        format!("{name}_{self}")
    }
    pub fn file_name(&self, name: &String) -> String {
        format!("{name}.webp", name = self.name(name))
    }
    pub fn file_format(&self) -> ImageFormat {
        ImageFormat::WebP
    }
    pub fn quality(&self) -> u32 {
        match self {
            ImageType::Normal => 100,
            ImageType::Thumbnail { quality, .. } => *quality,
        }
    }
    pub fn lossless(&self) -> bool {
        match self {
            ImageType::Normal => false,
            ImageType::Thumbnail { lossless, .. } => *lossless,
        }
    }
    pub fn extern_path(&self, name: &String) -> String {
        format!(
            "{EXTERN_LOCATION_IMAGES_STORAGE_PATH}{file_name}",
            file_name = self.file_name(&name)
        )
    }
    pub fn local_path(&self, name: &String) -> String {
        format!(
            "{LOCAL_IMAGES_STORAGE_PATH}{file_name}",
            file_name = self.file_name(&name)
        )
    }
    pub fn process_image(&self, image: DynamicImage) -> DynamicImage {
        match self {
            ImageType::Normal => image,
            ImageType::Thumbnail {
                nwidth, nheight, ..
            } => image.thumbnail(*nwidth, *nheight),
        }
    }
}

impl fmt::Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageType::Normal => write!(f, "normal"),
            ImageType::Thumbnail {
                nwidth,
                nheight,
                quality,
                lossless,
            } => {
                if *lossless {
                    write!(f, "thumbnail_{nwidth}_{nheight}_lossless_q{quality}")
                } else {
                    write!(f, "thumbnail_{nwidth}_{nheight}_q{quality}")
                }
            }
        }
    }
}
