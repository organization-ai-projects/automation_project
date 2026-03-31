use crate::error::ApFileError;

/// Pixel format for image data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PixelFormat {
    /// 8-bit grayscale (1 byte per pixel)
    Gray8 = 0,
    /// 24-bit RGB (3 bytes per pixel)
    Rgb8 = 1,
    /// 32-bit RGBA (4 bytes per pixel)
    Rgba8 = 2,
}

impl PixelFormat {
    /// Bytes per pixel for this format.
    pub fn bytes_per_pixel(self) -> usize {
        match self {
            Self::Gray8 => 1,
            Self::Rgb8 => 3,
            Self::Rgba8 => 4,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Gray8),
            1 => Some(Self::Rgb8),
            2 => Some(Self::Rgba8),
            _ => None,
        }
    }
}

/// Sub-header prepended to image payloads.
///
/// Layout (little-endian):
/// - width: u32 (4 bytes)
/// - height: u32 (4 bytes)
/// - pixel_format: u8 (1 byte)
/// - reserved: \[u8; 3\] (3 bytes)
///
/// Total: 12 bytes
#[derive(Debug, Clone, Copy)]
pub struct ImageHeader {
    pub width: u32,
    pub height: u32,
    pub pixel_format: PixelFormat,
}

impl ImageHeader {
    pub const SIZE: usize = 12;

    pub fn to_bytes(self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..4].copy_from_slice(&self.width.to_le_bytes());
        buf[4..8].copy_from_slice(&self.height.to_le_bytes());
        buf[8] = self.pixel_format as u8;
        // bytes 9..12 reserved
        buf
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ApFileError> {
        if bytes.len() < Self::SIZE {
            return Err(ApFileError::Corrupt("Image header too short"));
        }
        let width = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let height = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let pixel_format =
            PixelFormat::from_u8(bytes[8]).ok_or(ApFileError::Corrupt("Unknown pixel format"))?;
        Ok(Self {
            width,
            height,
            pixel_format,
        })
    }
}

/// Decoded image content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub pixel_format: PixelFormat,
    pub pixels: Vec<u8>,
}

impl ImageData {
    /// Create a new image, validating that `pixels` length matches dimensions.
    pub fn new(
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
        pixels: Vec<u8>,
    ) -> Result<Self, ApFileError> {
        let expected = width as usize * height as usize * pixel_format.bytes_per_pixel();
        if pixels.len() != expected {
            return Err(ApFileError::Encode(format!(
                "Pixel buffer length {} does not match {}x{}x{}={}",
                pixels.len(),
                width,
                height,
                pixel_format.bytes_per_pixel(),
                expected,
            )));
        }
        Ok(Self {
            width,
            height,
            pixel_format,
            pixels,
        })
    }

    /// Encode the image to a payload (image header + raw pixels).
    pub fn to_payload(&self) -> Vec<u8> {
        let header = ImageHeader {
            width: self.width,
            height: self.height,
            pixel_format: self.pixel_format,
        };
        let mut payload = Vec::with_capacity(ImageHeader::SIZE + self.pixels.len());
        payload.extend_from_slice(&header.to_bytes());
        payload.extend_from_slice(&self.pixels);
        payload
    }

    /// Decode image from a payload (image header + raw pixels).
    pub fn from_payload(payload: &[u8]) -> Result<Self, ApFileError> {
        let header = ImageHeader::from_bytes(payload)?;
        let pixel_data = &payload[ImageHeader::SIZE..];
        let expected =
            header.width as usize * header.height as usize * header.pixel_format.bytes_per_pixel();
        if pixel_data.len() != expected {
            return Err(ApFileError::Corrupt("Image pixel data size mismatch"));
        }
        Ok(Self {
            width: header.width,
            height: header.height,
            pixel_format: header.pixel_format,
            pixels: pixel_data.to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_header_round_trip() {
        let header = ImageHeader {
            width: 640,
            height: 480,
            pixel_format: PixelFormat::Rgba8,
        };
        let bytes = header.to_bytes();
        let decoded = ImageHeader::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.width, 640);
        assert_eq!(decoded.height, 480);
        assert_eq!(decoded.pixel_format, PixelFormat::Rgba8);
    }

    #[test]
    fn test_image_data_round_trip() {
        let pixels = vec![255u8; 2 * 2 * 3]; // 2x2 RGB
        let image = ImageData::new(2, 2, PixelFormat::Rgb8, pixels.clone()).unwrap();
        let payload = image.to_payload();
        let decoded = ImageData::from_payload(&payload).unwrap();
        assert_eq!(decoded, image);
    }

    #[test]
    fn test_image_data_size_mismatch() {
        let pixels = vec![0u8; 10]; // wrong length for 2x2 RGB
        let result = ImageData::new(2, 2, PixelFormat::Rgb8, pixels);
        assert!(result.is_err());
    }

    #[test]
    fn test_pixel_format_bytes() {
        assert_eq!(PixelFormat::Gray8.bytes_per_pixel(), 1);
        assert_eq!(PixelFormat::Rgb8.bytes_per_pixel(), 3);
        assert_eq!(PixelFormat::Rgba8.bytes_per_pixel(), 4);
    }

    #[test]
    fn test_grayscale_image() {
        let pixels = vec![128u8; 4 * 4]; // 4x4 grayscale
        let image = ImageData::new(4, 4, PixelFormat::Gray8, pixels.clone()).unwrap();
        let payload = image.to_payload();
        let decoded = ImageData::from_payload(&payload).unwrap();
        assert_eq!(decoded.width, 4);
        assert_eq!(decoded.height, 4);
        assert_eq!(decoded.pixel_format, PixelFormat::Gray8);
        assert_eq!(decoded.pixels, pixels);
    }
}
