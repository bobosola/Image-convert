# Image Convert

A simple command-line application which converts any supported image format to web-optimized JPG and WEBP formats.

I use this for making web-friendly images from AI-generated PNGs. It was entirely vibe-coded in Zed using the [Kimi CLI](https://github.com/MoonshotAI/kimi-cli) to run Kimi K2 via the Agent Client Protocol (ACP). I made it because I can never remember any of the command options for the existing CLI image conversion tools.

Its main purpose is to create images where the (smaller, faster) WebP image is the default, with the JPG image as fallback for older browsers:

```HTML
<figure>
   <picture>
       <source type="image/webp" srcset="images/myimage.webp">
       <img src="images/myimage.jpg" width="960" height="540" alt="Two people eating" loading="lazy">
   </picture>
   <figcaption><em>Bob and Karen eating out</em></figcaption>
</figure>
```

## Features

- Converts common image formats (see below) to web-optimized JPG and WebP
- Supports bulk conversion with wildcards like `*.png` or multiple files
- Uses lossy compression settings ideal for website performance (~75% quality)
- All converted images are saved in the same directory as the source image
- Preserves the original filename with the new extension
- Automatic format detection - no need to specify input format

## Supported Formats

- **Input**: Any format supported by the Rust `image` crate (PNG, JPG/JPEG, BMP, GIF, ICO, TIFF, WebP, and more)
- **Output**: JPG, WEBP

A notable exception is HEIF/HEIC images from Apple devices. But such devices allow for exporting HEIC photos as JPGs thus:
 - in the **Photos** App, select all the HEIC photos you want to convert - NB: ensure you do not select any videos or any other image types
 - hit the **Share** icon
 - then select **Copy Photos** - you won't see this option if you have selected a mix of media types
 - go the **Files** app and create a new folder
 - **long press** in a blank area of the folder, then select **paste** which will automatically convert your photos to JPGs

## Usage

```bash
image-convert <FILE_PATTERN> [FILE_PATTERN]...
```

Supports single files, multiple files, and wildcards like `*.png`

### Examples

**Single file conversion:**
```bash
image-convert image.png
# Creates: image.jpg and image.webp
```

**Multiple files:**
```bash
image-convert photo1.png photo2.jpg photo3.bmp
# Creates: photo1.jpg, photo1.webp, photo2.jpg, photo2.webp, etc.
```

**Wildcard/bulk conversion:**
```bash
image-convert *.png
# Converts all PNG files in current directory

image-convert images/*.jpg
# Converts all JPG files in images/ directory

image-convert "*.{png,jpg,bmp}"
# Bash-only: converts all PNG, JPG, and BMP files (using shell brace expansion)
```

## Compression Settings

Both JPG and WebP outputs use **lossy compression with default quality settings** (~75% quality):

- **JPG**: ~75% quality - provides excellent file size reduction with minimal visible quality loss
- **WebP**: ~75% quality lossy compression - typically 50-70% smaller than JPG at equivalent quality

These settings are **well suited for website use**, offering a good balance between:
✅ **Fast loading times** (significantly smaller file sizes)
✅ **Good visual quality** (minimal compression artifacts)
✅ **Broad compatibility** (all modern browsers support both formats)

**Note**: WebP encoding uses the `webp` crate for proper lossy compression, providing much better compression than the `image` crate's default lossless encoding.

## Error Handling

The application will display an error message if:
- The specified file does not exist
- The file format is not supported by the image crate
- There are issues during the conversion process

## Installation

[Install Rust](https://rust-lang.org/tools/install/), then:
```bash
git clone https://github.com/bobosola/image-convert
cd image-convert
cargo build --release
```

The binary will be available at `target/release/image-convert`.

## Dependencies

- `image` - Image processing library
- `clap` - Command-line argument parsing
- `webp` - webp encoding/decoding library
- `glob` - File pattern matching for wildcard support
