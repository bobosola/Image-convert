# Image Convert

A simple command-line application which
- converts PNG images to both JPG and WEBP formats
- converts JPG images to WEBP format

I use this for making web-friendly images from AI-generated PNGs. It was entirely vibe-coded in Zed using the  [Kimi CLI](https://github.com/MoonshotAI/kimi-cli) using Kimi K2 via the Agent Client Protocol (ACP) in about 30 seconds. The only reason I made it is because I can never remember any FFmpeg commands.

## Features

- All converted images are saved in the same directory as the source image
- Preserves the original filename with the new extension

## Usage

```bash
image-convert <FILE_PATH>
```

### Examples

Convert a PNG image:
```bash
image-convert image.png
# Creates: image.jpg and image.webp
```

Convert a JPG image:
```bash
image-convert photo.jpg
# Creates: photo.webp
```

## Supported Formats

- **Input**: PNG, JPG/JPEG
- **Output**: JPG, WEBP

## Error Handling

The application will display an error message if:
- The specified file does not exist
- The file format is not supported (only PNG and JPG are supported)
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
