# Image-convert WASM

Use ../src/lib.rs as the base code library to build a Rust WebAssembly-based single website page to convert common image files to WebP format. The aim is to have all the code, markup and style on one single HTML page. The entire site HTML, JS & CSS code should be viewable by viewing the source of the page in a browser and should not require any kind of packaging or similar tool.

The WASM code should work as described in ../README.md for the CLI application, but with the following differences:

* instead of a command-line input of image paths, the web page will use the JavaScript Files API to allow the user to select one or more input images by both a file uploader control and a drag and drop area on the page
* the output path for the converted files will always be the browser's downloads folder
* the option to additionally output a JPEG file will be an HTML checkbox defaulting to unchecked
* the option to alter the output quality will be an HTML slider control with a default position of 75%
* the option to resize an image shall be an HTML text input which only accepts a numeric value for width.

Include suitable descriptive text next to each control.

Write the website page in HTML5 with plain ES9/10 Javascript so that browsers from around 2020 onwards can use the page. The Javascript is to be written into a <script> element in the HTML page without using any external libraries or packing tools. Style the application with plain CSS all included in the HTML page's <style> element to give the page a clean modern look. 