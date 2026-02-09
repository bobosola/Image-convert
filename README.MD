# Image Convert

A fast and efficient Rust tool for converting images to web-optimized formats (WebP and JPEG). Available as both a CLI application and a web interface.

## Features

- **Two Interfaces**: Command-line tool and browser-based web interface
- **Wildcard Support**: Accepts file patterns like `*.png`, `images/**/*.jpg`
- **Multiple Formats**: Converts to WebP (default) and optionally JPEG
- **Image Resizing**: Resize images to maximum width while maintaining aspect ratio
- **Quality Control**: Adjustable quality settings for both WebP and JPEG
- **Batch Processing**: Process multiple images with automatic zipping for download
- **Smart Defaults**: Optimized quality defaults (WebP: 75, JPEG: 75) for web performance
- **Single HTML File**: Web interface is entirely self-contained in one file

## Project Structure

- **`src/lib.rs`** - Core image processing library (used by both CLI and web)
- **`src/main.rs`** - CLI application entry point
- **`web/index.htm`** - Self-contained web interface (HTML, CSS, JS, WASM)

## CLI Installation & Usage

### Installation

```bash
cargo build --release
```

The binary will be available at `target/release/image-convert`

### Basic Usage

Convert a single image to WebP:
```bash
image-convert photo.png
```

Convert multiple images with wildcards:
```bash
image-convert images/*.png photos/*.jpg
```

### CLI Options

- `-o, --output <DIR>`: Output directory (default: current directory)
- `-q, --quality <QUALITY>`: WebP quality 1-100 (default: 75)
- `-w, --max-width <WIDTH>`: Resize to maximum width
- `--jpeg`: Also convert to JPEG format
- `--jpeg-quality <QUALITY>`: JPEG quality 1-100 (default: 75)
- `-f, --force`: Overwrite existing files

### CLI Examples

Convert all PNGs to WebP with 85% quality:
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

## Web Interface

The web interface is a single HTML file that runs entirely in the browser using WebAssembly.

### Deployment

Only two files are needed to deploy the web interface:

1. `web/index.htm` - The web application (HTML, CSS, JavaScript)
2. `web/pkg/image_convert_bg.wasm` - The WebAssembly binary

Place both files in the same directory on your web server. The WASM file must be served with MIME type `application/wasm`.

### Web Features

- **Drag & Drop**: Drag images directly onto the page or click to browse
- **Batch Upload**: Select multiple files at once
- **Quality Slider**: Adjustable WebP quality (1-100, default 75%)
- **JPEG Option**: Checkbox to also generate JPEG versions
- **Resize**: Optional maximum width for all output images
- **Auto-Zip**: Multiple files are automatically zipped into a single download
- **Local Processing**: All conversion happens in your browser - no server processing

### Web Browser Compatibility

Works in all modern browsers (Chrome, Firefox, Safari, Edge) from approximately 2020 onwards.

## Supported Input Formats

- PNG
- JPEG/JPG
- GIF
- BMP
- TIFF/TIF
- WebP

## Output Formats

- **WebP**: Modern format with excellent compression (typically 20-30% smaller than JPEG at same quality)
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

### CLI Quality Adjustment

```bash
# Maximum quality for critical images
image-convert photo.png --quality 95

# Lower quality for thumbnails (even smaller files)
image-convert *.png --quality 65
```

## Technical Details

### Libraries Used

- `image` crate for image loading and basic operations
- `zenwebp` crate for WebP encoding (supports lossy compression with quality control)
- `glob` for efficient wildcard expansion (CLI)
- `wasm-bindgen` for WebAssembly/JavaScript interop (Web)
- `serde-wasm-bindgen` for data serialization (Web)

### Building the WebAssembly Module

```bash
# Install wasm-pack if not already installed
cargo install wasm-pack

# Build the WASM module
wasm-pack build --target web --out-dir web/pkg
```

The WASM binary is platform-independent and works on any OS (macOS, Linux, Windows) in any modern browser.

## License

MIT

## Acknowledgements

This was an experiment in 100% vibe-coding. I used [Zed](https://zed.dev) calling [Kimi CLI](https://github.com/MoonshotAI/kimi-cli) to access the **Kimi K2 Thinking** model. I later used the **Kimi 2.5** model to add the WASM feature. This acknowledgement section was the **only** thing I added throughout the whole process. The code and the rest of this README is exactly as written by the model other than me changing the application name from the AI-suggested one.
