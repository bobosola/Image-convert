use clap::Parser;
use image_convert::{expand_input_files, process_image, ProcessingOptions};
use std::path::PathBuf;

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

    // Expand wildcards and collect all input files
    let input_files = expand_input_files(&args.input)?;
    
    if input_files.is_empty() {
        eprintln!("No input files found!");
        std::process::exit(1);
    }

    println!("Found {} image file(s) to process", input_files.len());

    // Build processing options
    let options = ProcessingOptions {
        output_dir: args.output,
        webp_quality: args.quality,
        jpeg_quality: args.jpeg_quality,
        max_width: args.max_width,
        convert_to_jpeg: args.jpeg,
        force: args.force,
    };

    // Process each file
    let mut success_count = 0;
    let mut fail_count = 0;

    for input_path in input_files {
        match process_image(&input_path, &options) {
            Ok(result) => {
                println!("✓ Processed: {}", input_path.display());
                if let Some(webp_path) = result.webp_path {
                    println!("  → Created: {}", webp_path.display());
                }
                if let Some(jpeg_path) = result.jpeg_path {
                    println!("  → Created: {}", jpeg_path.display());
                }
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
