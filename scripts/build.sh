#!/bin/bash

# Build TailwindCSS
if [ ! -f "dist/tailwind.css" ]; then
  pnpm cross-env NODE_ENV=production postcss src/styles/main.css -o "dist/tailwind.css" --minify
fi

# Copy manifest.json (unhandled by Trunk)
cp src/assets/manifest.json dist/manifest.json

# Extract the hashed name of the file
HASH_512=$(ls dist/android-chrome-512x512-*.png | sed -e 's/^.*-\(.*\)\.png$/\1/')
HASH_192=$(ls dist/android-chrome-192x192-*.png | sed -e 's/^.*-\(.*\)\.png$/\1/')

# Replace the file names with the hashed names in the manifest.json file
sed -i '' "s/android-chrome-512x512.png/android-chrome-512x512-$HASH_512.png/g" dist/manifest.json
sed -i '' "s/android-chrome-192x192.png/android-chrome-192x192-$HASH_192.png/g" dist/manifest.json

# Copy robots.txt
cp src/assets/robots.txt dist/robots.txt
