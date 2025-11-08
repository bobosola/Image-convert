# Image Convert

`image-convert` creates web-optimised JPG and WebP copies of each input image. It has no options, flags or anything to remember. You just enter the file name(s) to be converted:

```bash
image-convert image.png
# or image-convert *.bmp, *.tiff, whatever
# or image-convert images/*.jpg
# or image-convert photo1.png photo2.jpg photo3.bmp
# or (for bash users) image-convert "*.{png,jpg,bmp}"
# or even just image-convert *
```
The JPG and WebP copies are created in the same location as the input images and have the same name as the original images.

> [!NOTE]
> If you feed `image-convert` existing JPG or WebP files, it will re-compress the original images which (unless they are already well optimised) will likely make them much smaller. In such cases I recommend working on safety copies of the original images to check that the image quality does not get unacceptably reduced.

I made `image-convert` because I can never remember any of the command options for the existing CLI image conversion tools. Its main purpose is to create images from non-web or large sized image formats for use in web pages where the (smaller, faster) WebP image is the default, with the JPG image as fallback for older browsers.

## Features

- Converts common image formats (see below) to web-optimized JPG and WebP
- Supports bulk conversion with wildcards like `*.png` or multiple files
- Uses lossy compression settings optimised for website performance (~75% quality)
- All converted images are saved in the same directory as the source image
- Preserves the original filename with the new extension
- Automatic format detection - no need to specify input format

## Supported Formats

- **Input**: Any format supported by the Rust `image` crate (PNG, JPG/JPEG, BMP, GIF, ICO, TIFF, WebP, and more)
- **Output**: JPG and WebP

A notable unsupported file type is HEIF/HEIC images from Apple devices. But such devices allow for the easy exporting of HEIC photos as JPGs thus:
 - in the **Photos** app, select all the HEIC photos you want to convert - NB: ensure you do not select any videos or any other image types
 - hit the **Share** icon (bottom left)
 - then select **Copy Photos** - you won't see this option if you have selected a mix of media types, it only appears if **all** the selected photos are HEICs
 - go the **Files** app and create a new folder
 - **long press** in a blank area of the folder, then select **paste** to automatically make JPG copies of the selected photos in the folder


## Compression Settings

Both JPG and WebP outputs use **lossy compression with default quality settings** (~75% quality):

- **JPG**: ~75% quality - provides excellent file size reduction with minimal visible quality loss
- **WebP**: ~75% quality lossy compression - typically 50-70% smaller than JPG at equivalent quality

These settings are **well suited for website use**, offering a good balance between:
- ✅ **Fast loading times** (significantly smaller file sizes)
- ✅ **Good visual quality** (minimal compression artifacts)
- ✅ **Broad compatibility** (all modern browsers support both formats)

## Error Handling

The application treats the following as errors:
- The specified file does not exist
- The file format is not supported by the `image` crate
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

- `image` - the main image processing library
- `clap` - command-line argument parsing
- `webp` - webp encoding/decoding library with lossy compression
- `glob` - file pattern matching for wildcard support

## Acknowledgements

This was an experiment in 100% vibe-coding. I used [Zed](https://zed.dev) calling  [Kimi CLI](https://github.com/MoonshotAI/kimi-cli) to access the Kimi K2 model. I did not examine the code _at all_ throughout the inital build, nor through the subsequent feature additions and changes. Kimi K2 even wrote and updated the README which was the **only** thing I manually edited in order to calm down its exuberance.
