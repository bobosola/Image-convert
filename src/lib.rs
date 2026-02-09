use image::{DynamicImage, GenericImageView, ImageFormat};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Cursor;

/// Options for image processing
#[derive(Debug, Clone)]
pub struct ProcessingOptions {
    /// Output directory
    pub output_dir: PathBuf,
    /// WebP quality (1-100)
    pub webp_quality: u8,
    /// JPEG quality (1-100)
    pub jpeg_quality: u8,
    /// Maximum width for resizing (maintains aspect ratio)
    pub max_width: Option<u32>,
    /// Also convert to JPEG format
    pub convert_to_jpeg: bool,
    /// Overwrite existing files
    pub force: bool,
}

impl Default for ProcessingOptions {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("."),
            webp_quality: 75,
            jpeg_quality: 75,
            max_width: None,
            convert_to_jpeg: false,
            force: false,
        }
    }
}

/// Result of processing a single image
#[derive(Debug)]
pub struct ProcessResult {
    pub input_path: PathBuf,
    pub webp_path: Option<PathBuf>,
    pub jpeg_path: Option<PathBuf>,
}

/// Error type for image processing operations
#[derive(Debug)]
pub enum ImageConvertError {
    Io(std::io::Error),
    Image(image::ImageError),
    Webp(String),
    InvalidFilename,
    Glob(glob::PatternError),
    GlobGlob(glob::GlobError),
}

impl std::fmt::Display for ImageConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageConvertError::Io(e) => write!(f, "IO error: {}", e),
            ImageConvertError::Image(e) => write!(f, "Image error: {}", e),
            ImageConvertError::Webp(s) => write!(f, "WebP error: {}", s),
            ImageConvertError::InvalidFilename => write!(f, "Invalid filename"),
            ImageConvertError::Glob(e) => write!(f, "Glob pattern error: {}", e),
            ImageConvertError::GlobGlob(e) => write!(f, "Glob error: {}", e),
        }
    }
}

impl std::error::Error for ImageConvertError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ImageConvertError::Io(e) => Some(e),
            ImageConvertError::Image(e) => Some(e),
            ImageConvertError::Glob(e) => Some(e),
            ImageConvertError::GlobGlob(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ImageConvertError {
    fn from(err: std::io::Error) -> Self {
        ImageConvertError::Io(err)
    }
}

impl From<image::ImageError> for ImageConvertError {
    fn from(err: image::ImageError) -> Self {
        ImageConvertError::Image(err)
    }
}

impl From<glob::PatternError> for ImageConvertError {
    fn from(err: glob::PatternError) -> Self {
        ImageConvertError::Glob(err)
    }
}

impl From<glob::GlobError> for ImageConvertError {
    fn from(err: glob::GlobError) -> Self {
        ImageConvertError::GlobGlob(err)
    }
}

/// Expand input patterns (supports wildcards like *.png)
pub fn expand_input_files(input_patterns: &[String]) -> Result<Vec<PathBuf>, ImageConvertError> {
    let mut files = Vec::new();

    for pattern in input_patterns {
        // Check if pattern contains wildcards
        if pattern.contains('*') || pattern.contains('?') || pattern.contains('[') {
            // Use glob to expand wildcards
            for entry in glob::glob(pattern)? {
                match entry {
                    Ok(path) => {
                        if is_supported_image(&path) {
                            files.push(path);
                        }
                    }
                    Err(e) => eprintln!("Error reading glob pattern {}: {}", pattern, e),
                }
            }
        } else {
            // No wildcards, treat as single file
            let path = PathBuf::from(pattern);
            if path.exists() && is_supported_image(&path) {
                files.push(path);
            }
        }
    }

    // Remove duplicates while preserving order
    let mut unique_files = Vec::new();
    for file in files {
        if !unique_files.contains(&file) {
            unique_files.push(file);
        }
    }

    Ok(unique_files)
}

/// Check if a path is a supported image file
pub fn is_supported_image(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    let supported_extensions = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "tif", "webp"];
    
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return supported_extensions.contains(&ext_str.to_lowercase().as_str());
        }
    }
    
    false
}

/// Process a single image file
pub fn process_image(
    input_path: &Path,
    options: &ProcessingOptions,
) -> Result<ProcessResult, ImageConvertError> {
    // Load the image
    let img = image::open(input_path)?;
    
    // Resize if max_width is specified
    let processed_img = if let Some(max_width) = options.max_width {
        resize_image(img, max_width)
    } else {
        img
    };

    // Get filename without extension
    let file_stem = input_path.file_stem()
        .ok_or(ImageConvertError::InvalidFilename)?
        .to_str()
        .ok_or(ImageConvertError::InvalidFilename)?;

    let mut result = ProcessResult {
        input_path: input_path.to_path_buf(),
        webp_path: None,
        jpeg_path: None,
    };

    // Create output directory if it doesn't exist
    if !options.output_dir.exists() {
        fs::create_dir_all(&options.output_dir)?;
    }

    // Convert to WebP
    let webp_path = options.output_dir.join(format!("{}.webp", file_stem));
    
    if !webp_path.exists() || options.force {
        save_as_webp(&processed_img, &webp_path, options.webp_quality)?;
        result.webp_path = Some(webp_path);
    }

    // Convert to JPEG if requested
    if options.convert_to_jpeg {
        let jpeg_path = options.output_dir.join(format!("{}.jpg", file_stem));
        
        if !jpeg_path.exists() || options.force {
            save_as_jpeg(&processed_img, &jpeg_path, options.jpeg_quality)?;
            result.jpeg_path = Some(jpeg_path);
        }
    }

    Ok(result)
}

/// Resize an image to a maximum width while maintaining aspect ratio
pub fn resize_image(img: DynamicImage, max_width: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    
    if width <= max_width {
        return img;
    }
    
    let ratio = max_width as f32 / width as f32;
    let new_height = (height as f32 * ratio) as u32;
    
    img.resize(max_width, new_height, image::imageops::FilterType::Lanczos3)
}

/// Save an image as WebP format using pure Rust encoder
pub fn save_as_webp(
    img: &DynamicImage,
    path: &Path,
    quality: u8,
) -> Result<(), ImageConvertError> {
    let webp_bytes = image_to_webp_bytes(img, quality)?;
    fs::write(path, webp_bytes)?;
    Ok(())
}

/// Save an image as JPEG format
pub fn save_as_jpeg(
    img: &DynamicImage,
    path: &Path,
    quality: u8,
) -> Result<(), ImageConvertError> {
    let jpeg_bytes = image_to_jpeg_bytes(img, quality)?;
    fs::write(path, jpeg_bytes)?;
    Ok(())
}

/// Convert image to WebP bytes (in memory) using lossy encoding
fn image_to_webp_bytes(
    img: &DynamicImage,
    quality: u8,
) -> Result<Vec<u8>, ImageConvertError> {
    use zenwebp::{EncodeRequest, LossyConfig, PixelLayout};
    
    let quality = quality.clamp(1, 100) as f32;
    
    // Convert to RGB for WebP encoding (WebP uses YUV internally, RGB is more efficient)
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    
    // Configure lossy encoding with quality and preset for better compression
    let config = LossyConfig::new()
        .with_quality(quality)
        .with_method(6) // Higher method = better compression, slower (0-6)
        .with_preset_value(zenwebp::Preset::Photo); // Optimized for photos
    
    // Encode to WebP
    let webp_data = EncodeRequest::lossy(
        &config,
        &rgb_img,
        PixelLayout::Rgb8,
        width,
        height
    )
    .encode()
    .map_err(|e| ImageConvertError::Webp(format!("WebP encoding failed: {:?}", e)))?;
    
    Ok(webp_data.to_vec())
}

/// Convert image to JPEG bytes (in memory)
fn image_to_jpeg_bytes(
    img: &DynamicImage,
    _quality: u8,
) -> Result<Vec<u8>, ImageConvertError> {
    let mut buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Jpeg)?;
    Ok(buffer)
}

// ============================================================================
// WASM-specific code
// ============================================================================

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

/// Initialize panic hook for better error messages in browser console
#[wasm_bindgen(start)]
pub fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// WASM-friendly result structure
#[derive(Serialize, Deserialize)]
pub struct WasmConvertResult {
    pub filename: String,
    pub webp_data: Option<Vec<u8>>,
    pub jpeg_data: Option<Vec<u8>>,
    pub error: Option<String>,
}

/// Convert image bytes to WebP (and optionally JPEG) in WASM
/// 
/// # Arguments
/// * `image_bytes` - Raw bytes of the input image
/// * `filename` - Original filename (used to determine output name)
/// * `webp_quality` - WebP quality 1-100
/// * `jpeg_quality` - JPEG quality 1-100
/// * `max_width` - Optional maximum width for resizing
/// * `convert_to_jpeg` - Whether to also convert to JPEG
#[wasm_bindgen]
pub fn convert_image(
    image_bytes: &[u8],
    filename: &str,
    webp_quality: u8,
    jpeg_quality: u8,
    max_width: Option<u32>,
    convert_to_jpeg: bool,
) -> JsValue {
    let result = convert_image_internal(
        image_bytes,
        filename,
        webp_quality,
        jpeg_quality,
        max_width,
        convert_to_jpeg,
    );
    
    match result {
        Ok(converted) => serde_wasm_bindgen::to_value(&converted).unwrap(),
        Err(e) => {
            let error_result = WasmConvertResult {
                filename: filename.to_string(),
                webp_data: None,
                jpeg_data: None,
                error: Some(e.to_string()),
            };
            serde_wasm_bindgen::to_value(&error_result).unwrap()
        }
    }
}

fn convert_image_internal(
    image_bytes: &[u8],
    filename: &str,
    webp_quality: u8,
    jpeg_quality: u8,
    max_width: Option<u32>,
    convert_to_jpeg: bool,
) -> Result<WasmConvertResult, ImageConvertError> {
    // Load image from bytes
    let img = image::load_from_memory(image_bytes)?;
    
    // Resize if max_width is specified
    let processed_img = if let Some(max_width) = max_width {
        resize_image(img, max_width)
    } else {
        img
    };

    // Get filename stem (without extension)
    let file_stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("converted");

    let mut result = WasmConvertResult {
        filename: file_stem.to_string(),
        webp_data: None,
        jpeg_data: None,
        error: None,
    };

    // Convert to WebP
    let webp_bytes = image_to_webp_bytes(&processed_img, webp_quality)?;
    result.webp_data = Some(webp_bytes);

    // Convert to JPEG if requested
    if convert_to_jpeg {
        let jpeg_bytes = image_to_jpeg_bytes(&processed_img, jpeg_quality)?;
        result.jpeg_data = Some(jpeg_bytes);
    }

    Ok(result)
}

/// Check if a filename has a supported image extension (WASM version - no filesystem check)
#[wasm_bindgen]
pub fn is_supported_image_file(filename: &str) -> bool {
    let path = Path::new(filename);
    
    let supported_extensions = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "tif", "webp"];
    
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return supported_extensions.contains(&ext_str.to_lowercase().as_str());
        }
    }
    
    false
}
