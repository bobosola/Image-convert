# Image Convert

A fast and efficient Rust CLI tool for converting images to web-optimized formats (WebP and JPEG).

## Features

- **Wildcard Support**: Accepts file patterns like `*.png`, `images/**/*.jpg`
- **Multiple Formats**: Converts to WebP (default) and optionally JPEG
- **Image Resizing**: Resize images to maximum width while maintaining aspect ratio
- **Quality Control**: Adjustable quality settings for both WebP and JPEG
- **Batch Processing**: Process multiple images in a single command
- **Smart Defaults**: Optimized quality defaults (WebP: 75, JPEG: 75) for web performance

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/image-convert`

## Usage

### Basic Usage

Convert a single image to WebP:
```bash
image-convert photo.png
```

Convert multiple images with wildcards:
```bash
image-convert images/*.png photos/*.jpg
```

### Options

- `-o, --output <DIR>`: Output directory (default: current directory)
- `-q, --quality <QUALITY>`: WebP quality 1-100 (default: 75)
- `-w, --max-width <WIDTH>`: Resize to maximum width
- `--jpeg`: Also convert to JPEG format
- `--jpeg-quality <QUALITY>`: JPEG quality 1-100 (default: 75)
- `-f, --force`: Overwrite existing files

### Examples

Convert all PNGs to WebP with 85% quality (higher quality):
```bash
image-convert *.png --quality 85
```

Convert images and resize to 800px width:
```bash
image-convert photos/* --max-width 800 --output optimized/
```

Convert to both WebP and JPEG:
```bash
image-convert image.png --jpeg --quality 80 --jpeg-quality 80
```

Process with wildcards from different directories:
```bash
image-convert assets/images/*.png assets/photos/*.jpg --output dist/
```

## Supported Input Formats

- PNG
- JPEG/JPG
- GIF
- BMP
- TIFF/TIF
- WebP

## Output Formats

- **WebP**: Modern format with excellent compression
- **JPEG**: Widely supported format (optional)

## Quality Settings

### Why 75% is the Default

The default quality of **75%** is optimized for web performance while maintaining excellent visual quality. Based on testing, here's what this means:

**File Size Savings:**
- 75% quality produces files that are **39% smaller** than 85% quality
- A typical 2.7MB JPEG becomes ~0.7MB at 75% WebP (74% reduction)
- Compared to 85% WebP (~1.2MB), you save an additional 476KB per image

**Visual Quality:**
- Most users cannot distinguish 75% from 85% on typical screens
- Differences are only visible when zooming in or on very large displays
- Perfect for web galleries, e-commerce, blogs, and social media

**Performance Benefits:**
- Faster page loads and better user experience
- Reduced bandwidth costs (significant for high-traffic sites)
- Improved mobile performance
- Better SEO scores (Core Web Vitals)

**When to Use Higher Quality:**
- Photography portfolios where visual fidelity is critical
- Artwork, medical imaging, or print preparation
- Large 4K/5K displays where artifacts are more visible

You can easily adjust quality based on your needs:
```bash
# Maximum quality for critical images
image-convert photo.png --quality 95

# Lower quality for thumbnails (even smaller files)
image-convert *.png --quality 65
```

## Performance

The tool uses optimized Rust libraries for fast image processing:
- `image` crate for image loading and basic operations
- `webp` crate for WebP encoding
- `glob` for efficient wildcard expansion
- Parallel processing ready (can be extended with Rayon)

## License

MIT


## Acknowledgements

This was an experiment in 100% vibe-coding. I used [Zed](https://zed.dev) calling  [Kimi CLI](https://github.com/MoonshotAI/kimi-cli) to access the **Kimi K2 Thinking** model. This acknowledgement section was the **only** thing I added throughout the whole process. The code and the rest of this README is exactly as written by the model, other than me changing the name from _image-optimizer_ to _image-convert_ to avoid having to remember whether it was _optimizer_ with a _z_ (USA style) or _optimiser_ with an _s_ (UK style).
