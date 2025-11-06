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
    
    for pattern in &args.file_patterns {
        // Check if it's a simple file (no wildcards)
        if !pattern.contains('*') && !pattern.contains('?') && !pattern.contains('[') {
            let path = PathBuf::from(pattern);
            if path.exists() {
                files_to_process.push(path);
            } else {
                eprintln!("Warning: File '{}' does not exist", pattern);
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
                            Err(e) => eprintln!("Warning: Error accessing path: {}", e),
                        }
                    }
                    if !found_files {
                        eprintln!("Warning: No files found matching pattern '{}'", pattern);
                    }
                }
                Err(e) => eprintln!("Warning: Invalid pattern '{}': {}", pattern, e),
            }
        }
    }
    
    if files_to_process.is_empty() {
        return Err("No valid files found to process".into());
    }
    
    // Process all files
    println!("Found {} file(s) to convert", files_to_process.len());
    
    for (i, file_path) in files_to_process.iter().enumerate() {
        println!("\nProcessing file {} of {}: {}", i + 1, files_to_process.len(), file_path.display());
        
        if let Err(e) = convert_image(file_path) {
            eprintln!("Error converting '{}': {}", file_path.display(), e);
            // Continue with other files even if one fails
        }
    }
    
    println!("\n✅ Bulk conversion complete!");
    Ok(())
}

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