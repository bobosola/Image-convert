use clap::Parser;
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Parser, Debug)]
#[command(name = "image-optimizer")]
#[command(about = "Convert images to web-optimized formats")]
struct Args {
    /// Input image files (supports wildcards like *.png)
    #[arg(required = true)]
    input: Vec<String>,

    /// Output directory (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    output: PathBuf,

    /// WebP quality (1-100, higher is better quality)
    #[arg(short = 'q', long, default_value = "75")]
    quality: u8,

    /// Resize images to maximum width (maintains aspect ratio)
    #[arg(short = 'w', long)]
    max_width: Option<u32>,

    /// Also convert to JPEG format
    #[arg(long)]
    jpeg: bool,

    /// JPEG quality (1-100)
    #[arg(long, default_value = "75")]
    jpeg_quality: u8,

    /// Overwrite existing files
    #[arg(short, long)]
    force: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Create output directory if it doesn't exist
    if !args.output.exists() {
        fs::create_dir_all(&args.output)?;
    }

    // Expand wildcards and collect all input files
    let input_files = expand_input_files(&args.input)?;
    
    if input_files.is_empty() {
        eprintln!("No input files found!");
        std::process::exit(1);
    }

    println!("Found {} image file(s) to process", input_files.len());

    // Process each file
    let mut success_count = 0;
    let mut fail_count = 0;

    for input_path in input_files {
        match process_image(&input_path, &args) {
            Ok(_) => {
                println!("✓ Processed: {}", input_path.display());
                success_count += 1;
            }
            Err(e) => {
                eprintln!("✗ Failed to process {}: {}", input_path.display(), e);
                fail_count += 1;
            }
        }
    }

    println!("\nSummary: {} succeeded, {} failed", success_count, fail_count);
    
    if fail_count > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn expand_input_files(input_patterns: &[String]) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
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

fn is_supported_image(path: &Path) -> bool {
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

fn process_image(input_path: &Path, args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    // Load the image
    let img = image::open(input_path)?;
    
    // Resize if max_width is specified
    let processed_img = if let Some(max_width) = args.max_width {
        resize_image(img, max_width)
    } else {
        img
    };

    // Get filename without extension
    let file_stem = input_path.file_stem()
        .ok_or("Invalid filename")?
        .to_str()
        .ok_or("Invalid filename encoding")?;

    // Convert to WebP
    let webp_path = args.output.join(format!("{}.webp", file_stem));
    
    if !webp_path.exists() || args.force {
        save_as_webp(&processed_img, &webp_path, args.quality)?;
        println!("  → Created: {}", webp_path.display());
    } else {
        println!("  → Skipped (already exists): {}", webp_path.display());
    }

    // Convert to JPEG if requested
    if args.jpeg {
        let jpeg_path = args.output.join(format!("{}.jpg", file_stem));
        
        if !jpeg_path.exists() || args.force {
            save_as_jpeg(&processed_img, &jpeg_path, args.jpeg_quality)?;
            println!("  → Created: {}", jpeg_path.display());
        } else {
            println!("  → Skipped (already exists): {}", jpeg_path.display());
        }
    }

    Ok(())
}

fn resize_image(img: DynamicImage, max_width: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    
    if width <= max_width {
        return img;
    }
    
    let ratio = max_width as f32 / width as f32;
    let new_height = (height as f32 * ratio) as u32;
    
    img.resize(max_width, new_height, image::imageops::FilterType::Lanczos3)
}

fn save_as_webp(img: &DynamicImage, path: &Path, quality: u8) -> Result<(), Box<dyn std::error::Error>> {
    // Clamp quality to valid range
    let quality = quality.clamp(1, 100);
    
    // Convert to RGBA8 for webp encoding
    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();
    
    // Create WebP encoder
    let encoder = webp::Encoder::from_rgba(&rgba_img, width, height);
    let webp_data = encoder.encode(quality as f32);
    
    // Write to file
    fs::write(path, &*webp_data)?;
    
    Ok(())
}

fn save_as_jpeg(img: &DynamicImage, path: &Path, quality: u8) -> Result<(), Box<dyn std::error::Error>> {
    // Clamp quality to valid range
    let _quality = quality.clamp(1, 100);
    
    // Save as JPEG
    let mut output_file = fs::File::create(path)?;
    img.write_to(&mut output_file, ImageFormat::Jpeg)?;
    
    Ok(())
}