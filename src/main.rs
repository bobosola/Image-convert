use clap::Parser;
use image::ImageFormat;
use std::path::{Path, PathBuf};
use webp::Encoder;
use glob::glob;

#[derive(Parser, Debug)]
#[command(name = "image-convert")]
#[command(about = "Convert any supported image format to JPG and WebP")]
struct Args {
    #[arg(help = "Path(s) to image files - can be single file, multiple files, or wildcards like *.png")]
    file_patterns: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    if args.file_patterns.is_empty() {
        return Err("No file patterns specified. Use: image-convert <file(s)> or image-convert *.png".into());
    }
    
    // Collect all files from the provided patterns
    let mut files_to_process = Vec::new();
    let mut missing_files = 0;
    let mut invalid_patterns = 0;
    
    for pattern in &args.file_patterns {
        // Check if it's a simple file (no wildcards)
        if !pattern.contains('*') && !pattern.contains('?') && !pattern.contains('[') {
            let path = PathBuf::from(pattern);
            if path.exists() {
                files_to_process.push(path);
            } else {
                missing_files += 1;
            }
        } else {
            // It's a wildcard pattern - use glob
            match glob(pattern) {
                Ok(paths) => {
                    let mut found_files = false;
                    for entry in paths {
                        match entry {
                            Ok(path) => {
                                if path.is_file() {
                                    files_to_process.push(path);
                                    found_files = true;
                                }
                            }
                            Err(_) => {
                                // Individual path error, continue with other matches
                            }
                        }
                    }
                    if !found_files {
                        // Pattern matched no files - this is different from pattern syntax error
                    }
                }
                Err(_) => {
                    invalid_patterns += 1;
                }
            }
        }
    }
    
    if files_to_process.is_empty() {
        return Err("No valid files found to process".into());
    }
    
    // Track conversion statistics
    let total_files = files_to_process.len();
    let mut converted_count = 0;
    let mut unsupported_count = 0;
    let mut error_count = 0;
    
    // Process all files with minimal output
    for file_path in &files_to_process {
        match convert_image_quiet(file_path) {
            Ok(()) => {
                converted_count += 1;
            }
            Err(e) => {
                if e.to_string().contains("Unsupported") || e.to_string().contains("Decoding") {
                    unsupported_count += 1;
                } else {
                    error_count += 1;
                }
            }
        }
    }
    
    // Print summary
    let _total_attempted = missing_files + invalid_patterns + total_files;
    println!("\n📊 Conversion Summary:");
    println!("  Converted: {} of {} files", converted_count, total_files);
    
    if unsupported_count > 0 {
        println!("  Ignored: {} unsupported file format(s)", unsupported_count);
    }
    if missing_files > 0 {
        println!("  Missing: {} file(s) not found", missing_files);
    }
    if invalid_patterns > 0 {
        println!("  Invalid: {} pattern(s)", invalid_patterns);
    }
    if error_count > 0 {
        println!("  Errors: {} file(s) failed to convert", error_count);
    }
    
    println!("\n✅ Conversion complete!");
    Ok(())
}

#[allow(dead_code)]
fn convert_image(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Converting image file: {}", file_path.display());
    
    let img = image::open(file_path)?;
    let parent_dir = file_path.parent().unwrap_or(Path::new("."));
    let file_stem = file_path.file_stem().unwrap_or_default().to_str().unwrap_or("output");
    
    // Convert to JPG
    let jpg_path = parent_dir.join(format!("{}.jpg", file_stem));
    img.save_with_format(&jpg_path, ImageFormat::Jpeg)?;
    println!("Created JPG: {}", jpg_path.display());
    
    // Convert to WEBP using lossy compression with ~75% quality
    let webp_path = parent_dir.join(format!("{}.webp", file_stem));
    
    // Convert image to RGB for WebP encoding
    let rgb_img = img.to_rgb8();
    let (width, height) = (rgb_img.width() as u32, rgb_img.height() as u32);
    
    // Create WebP encoder with lossy compression
    let encoder = Encoder::from_rgb(&rgb_img, width, height);
    let webp_data = encoder.encode(75.0); // 75% quality for optimal web performance
    
    // Write the WebP file
    std::fs::write(&webp_path, webp_data.as_ref())?;
    println!("Created WebP: {} (lossy, 75% quality)", webp_path.display());
    
    Ok(())
}

fn convert_image_quiet(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(file_path)?;
    let parent_dir = file_path.parent().unwrap_or(Path::new("."));
    let file_stem = file_path.file_stem().unwrap_or_default().to_str().unwrap_or("output");
    
    // Convert to JPG
    let jpg_path = parent_dir.join(format!("{}.jpg", file_stem));
    img.save_with_format(&jpg_path, ImageFormat::Jpeg)?;
    
    // Convert to WEBP using lossy compression with ~75% quality
    let webp_path = parent_dir.join(format!("{}.webp", file_stem));
    
    // Convert image to RGB for WebP encoding
    let rgb_img = img.to_rgb8();
    let (width, height) = (rgb_img.width() as u32, rgb_img.height() as u32);
    
    // Create WebP encoder with lossy compression
    let encoder = Encoder::from_rgb(&rgb_img, width, height);
    let webp_data = encoder.encode(75.0); // 75% quality for optimal web performance
    
    // Write the WebP file
    std::fs::write(&webp_path, webp_data.as_ref())?;
    
    // Only show the converted files, no verbose messages
    println!("  {} → {} + {}", file_path.display(), jpg_path.display(), webp_path.display());
    
    Ok(())
}