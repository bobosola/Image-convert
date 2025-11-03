use clap::Parser;
use image::ImageFormat;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "image-convert")]
#[command(about = "Convert PNG/JPG images to other formats")]
struct Args {
    #[arg(help = "Path to the image file (PNG or JPG)")]
    file_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let file_path = args.file_path;
    
    if !file_path.exists() {
        return Err(format!("File '{}' does not exist", file_path.display()).into());
    }
    
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());
    
    match extension.as_deref() {
        Some("png") => convert_png(&file_path)?,
        Some("jpg") | Some("jpeg") => convert_jpg(&file_path)?,
        _ => return Err(format!("Unsupported file format. Only PNG and JPG files are supported.").into()),
    }
    
    Ok(())
}

fn convert_png(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Converting PNG file: {}", file_path.display());
    
    let img = image::open(file_path)?;
    let parent_dir = file_path.parent().unwrap_or(Path::new("."));
    let file_stem = file_path.file_stem().unwrap_or_default().to_str().unwrap_or("output");
    
    // Convert to JPG
    let jpg_path = parent_dir.join(format!("{}.jpg", file_stem));
    img.save_with_format(&jpg_path, ImageFormat::Jpeg)?;
    println!("Created JPG: {}", jpg_path.display());
    
    // Convert to WEBP
    let webp_path = parent_dir.join(format!("{}.webp", file_stem));
    img.save_with_format(&webp_path, ImageFormat::WebP)?;
    println!("Created WEBP: {}", webp_path.display());
    
    Ok(())
}

fn convert_jpg(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Converting JPG file: {}", file_path.display());
    
    let img = image::open(file_path)?;
    let parent_dir = file_path.parent().unwrap_or(Path::new("."));
    let file_stem = file_path.file_stem().unwrap_or_default().to_str().unwrap_or("output");
    
    // Convert to WEBP
    let webp_path = parent_dir.join(format!("{}.webp", file_stem));
    img.save_with_format(&webp_path, ImageFormat::WebP)?;
    println!("Created WEBP: {}", webp_path.display());
    
    Ok(())
}