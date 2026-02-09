#!/opt/homebrew/bin/bash
# 
# 1) Builds the CLI image-convert app in target/release
#    and also build the WASM version in web/pkg.
# 2) Then deploys the WASM version plus HTML page to
#    a local website directory for testing on a local
#    web server.
# 
# WASM Cache Busting: Update the version parameter in the HTML when 
# deploying new builds. Search for: const WASM_VERSION = 'v=1.0.0';

# Change paths to suit (WEBSITE_PATH must exist)
WEBSITE_PATH=/Users/bobosola/Sites/osola.org.uk/tools
DEPLOY_DIR=imageconvert

FULLPATH="$WEBSITE_PATH/$DEPLOY_DIR"
if [ ! -d "$FULLPATH" ]; then
  mkdir "$FULLPATH"
fi
cargo build --release
cp web/index.htm web/pkg/image_convert_bg.wasm "$FULLPATH"