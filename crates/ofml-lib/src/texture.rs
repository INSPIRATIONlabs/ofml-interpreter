//! Texture Loading System - Image file handling for materials.
//!
//! This module implements texture loading for OFML products including:
//! - PNG, JPG, TGA, BMP format support
//! - Texture data management for GLB export
//! - Image format conversion

use std::collections::HashMap;
use std::io::Cursor;

use image::{DynamicImage, GenericImageView, ImageFormat};

use crate::errors::MaterialError;

/// Loaded texture data ready for export.
#[derive(Debug, Clone)]
pub struct TextureData {
    /// Texture name/identifier
    pub name: String,
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
    /// PNG-encoded image data (for GLB embedding)
    pub png_data: Vec<u8>,
    /// Original file format
    pub original_format: ImageFormat,
}

impl TextureData {
    /// Create a new texture from raw image bytes.
    pub fn from_bytes(name: impl Into<String>, data: &[u8]) -> Result<Self, MaterialError> {
        let name = name.into();

        // Detect format
        let format = image::guess_format(data).map_err(|e| MaterialError::TextureLoadError {
            filename: name.clone(),
            message: format!("Could not detect image format: {}", e),
        })?;

        // Load image
        let img = image::load_from_memory(data).map_err(|e| MaterialError::TextureLoadError {
            filename: name.clone(),
            message: format!("Failed to decode image: {}", e),
        })?;

        // Convert to PNG for GLB embedding
        let (width, height) = img.dimensions();
        let png_data = encode_as_png(&img)?;

        Ok(Self {
            name,
            width,
            height,
            png_data,
            original_format: format,
        })
    }

    /// Create a new texture from a dynamic image.
    pub fn from_image(name: impl Into<String>, img: DynamicImage) -> Result<Self, MaterialError> {
        let name = name.into();
        let (width, height) = img.dimensions();
        let png_data = encode_as_png(&img)?;

        Ok(Self {
            name,
            width,
            height,
            png_data,
            original_format: ImageFormat::Png,
        })
    }

    /// Get the MIME type for GLB embedding.
    pub fn mime_type(&self) -> &'static str {
        "image/png" // Always PNG for GLB
    }

    /// Get the byte size of the texture data.
    pub fn byte_size(&self) -> usize {
        self.png_data.len()
    }
}

/// Encode an image as PNG.
fn encode_as_png(img: &DynamicImage) -> Result<Vec<u8>, MaterialError> {
    let mut png_data = Vec::new();
    let mut cursor = Cursor::new(&mut png_data);

    img.write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| MaterialError::InvalidProperty {
            property: "png_encode".to_string(),
            message: format!("Failed to encode as PNG: {}", e),
        })?;

    Ok(png_data)
}

/// Texture cache for managing loaded textures.
#[derive(Debug, Default)]
pub struct TextureCache {
    /// Cached textures by name
    textures: HashMap<String, TextureData>,
}

impl TextureCache {
    /// Create a new empty texture cache.
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    /// Load and cache a texture from bytes.
    pub fn load(&mut self, name: &str, data: &[u8]) -> Result<&TextureData, MaterialError> {
        if !self.textures.contains_key(name) {
            let texture = TextureData::from_bytes(name, data)?;
            self.textures.insert(name.to_string(), texture);
        }
        Ok(self.textures.get(name).unwrap())
    }

    /// Get a cached texture by name.
    pub fn get(&self, name: &str) -> Option<&TextureData> {
        self.textures.get(name)
    }

    /// Check if a texture is cached.
    pub fn contains(&self, name: &str) -> bool {
        self.textures.contains_key(name)
    }

    /// Insert a pre-loaded texture.
    pub fn insert(&mut self, texture: TextureData) {
        self.textures.insert(texture.name.clone(), texture);
    }

    /// Get all cached texture names.
    pub fn names(&self) -> Vec<&String> {
        self.textures.keys().collect()
    }

    /// Get all cached textures.
    pub fn all(&self) -> impl Iterator<Item = &TextureData> {
        self.textures.values()
    }

    /// Get the number of cached textures.
    pub fn len(&self) -> usize {
        self.textures.len()
    }

    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }

    /// Clear the cache.
    pub fn clear(&mut self) {
        self.textures.clear();
    }
}

/// Create a solid color texture.
pub fn create_solid_color(
    name: &str,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) -> Result<TextureData, MaterialError> {
    let img = DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
        1,
        1,
        image::Rgba([r, g, b, a]),
    ));
    TextureData::from_image(name, img)
}

/// Create a checkerboard texture (useful for debugging).
pub fn create_checkerboard(
    name: &str,
    size: u32,
    color1: [u8; 4],
    color2: [u8; 4],
) -> Result<TextureData, MaterialError> {
    let mut img = image::RgbaImage::new(size, size);

    for y in 0..size {
        for x in 0..size {
            let color = if ((x / 8) + (y / 8)) % 2 == 0 {
                image::Rgba(color1)
            } else {
                image::Rgba(color2)
            };
            img.put_pixel(x, y, color);
        }
    }

    TextureData::from_image(name, DynamicImage::ImageRgba8(img))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_color_texture() {
        let tex = create_solid_color("red", 255, 0, 0, 255).unwrap();
        assert_eq!(tex.name, "red");
        assert_eq!(tex.width, 1);
        assert_eq!(tex.height, 1);
        assert!(!tex.png_data.is_empty());
    }

    #[test]
    fn test_checkerboard_texture() {
        let tex = create_checkerboard("check", 64, [255, 255, 255, 255], [0, 0, 0, 255]).unwrap();
        assert_eq!(tex.name, "check");
        assert_eq!(tex.width, 64);
        assert_eq!(tex.height, 64);
    }

    #[test]
    fn test_texture_cache() {
        let mut cache = TextureCache::new();
        assert!(cache.is_empty());

        let tex = create_solid_color("test", 128, 128, 128, 255).unwrap();
        cache.insert(tex);

        assert_eq!(cache.len(), 1);
        assert!(cache.contains("test"));
        assert!(cache.get("test").is_some());
    }

    #[test]
    fn test_mime_type() {
        let tex = create_solid_color("test", 0, 0, 0, 255).unwrap();
        assert_eq!(tex.mime_type(), "image/png");
    }

    #[test]
    fn test_texture_data_byte_size() {
        let tex = create_solid_color("test", 255, 0, 0, 255).unwrap();
        assert!(tex.byte_size() > 0);
        assert_eq!(tex.byte_size(), tex.png_data.len());
    }

    #[test]
    fn test_texture_cache_names() {
        let mut cache = TextureCache::new();
        let tex1 = create_solid_color("tex1", 255, 0, 0, 255).unwrap();
        let tex2 = create_solid_color("tex2", 0, 255, 0, 255).unwrap();
        cache.insert(tex1);
        cache.insert(tex2);

        let names = cache.names();
        assert_eq!(names.len(), 2);
    }

    #[test]
    fn test_texture_cache_all() {
        let mut cache = TextureCache::new();
        let tex = create_solid_color("test", 128, 128, 128, 255).unwrap();
        cache.insert(tex);

        let textures: Vec<_> = cache.all().collect();
        assert_eq!(textures.len(), 1);
        assert_eq!(textures[0].name, "test");
    }

    #[test]
    fn test_texture_cache_clear() {
        let mut cache = TextureCache::new();
        let tex = create_solid_color("test", 0, 0, 0, 255).unwrap();
        cache.insert(tex);
        assert!(!cache.is_empty());

        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_texture_data_from_bytes_invalid() {
        let invalid_data = vec![0, 1, 2, 3, 4, 5];
        let result = TextureData::from_bytes("invalid", &invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_texture_cache_get_missing() {
        let cache = TextureCache::new();
        assert!(cache.get("nonexistent").is_none());
        assert!(!cache.contains("nonexistent"));
    }

    #[test]
    fn test_texture_data_clone() {
        let tex = create_solid_color("original", 100, 150, 200, 255).unwrap();
        let cloned = tex.clone();
        assert_eq!(cloned.name, "original");
        assert_eq!(cloned.width, 1);
        assert_eq!(cloned.height, 1);
    }

    #[test]
    fn test_texture_data_debug() {
        let tex = create_solid_color("test", 0, 0, 0, 255).unwrap();
        let debug_str = format!("{:?}", tex);
        assert!(debug_str.contains("TextureData"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_texture_cache_default() {
        let cache = TextureCache::default();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_texture_cache_debug() {
        let cache = TextureCache::new();
        let debug_str = format!("{:?}", cache);
        assert!(debug_str.contains("TextureCache"));
    }

    #[test]
    fn test_texture_data_from_bytes_png() {
        // Create a simple 1x1 red PNG using image crate
        let img = DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
            1,
            1,
            image::Rgba([255, 0, 0, 255]),
        ));
        let mut png_bytes = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png_bytes);
        img.write_to(&mut cursor, ImageFormat::Png).unwrap();

        let tex = TextureData::from_bytes("test_png", &png_bytes).unwrap();
        assert_eq!(tex.name, "test_png");
        assert_eq!(tex.width, 1);
        assert_eq!(tex.height, 1);
        assert_eq!(tex.original_format, ImageFormat::Png);
    }

    #[test]
    fn test_texture_cache_load() {
        // Create a simple PNG
        let img = DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
            1,
            1,
            image::Rgba([0, 255, 0, 255]),
        ));
        let mut png_bytes = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png_bytes);
        img.write_to(&mut cursor, ImageFormat::Png).unwrap();

        let mut cache = TextureCache::new();
        let result = cache.load("green", &png_bytes);
        assert!(result.is_ok());

        // Loading again should return cached version
        let result2 = cache.load("green", &png_bytes);
        assert!(result2.is_ok());
        assert_eq!(cache.len(), 1); // Still just 1 texture
    }

    #[test]
    fn test_texture_cache_load_invalid() {
        let mut cache = TextureCache::new();
        let result = cache.load("invalid", &[1, 2, 3, 4]);
        assert!(result.is_err());
    }
}
