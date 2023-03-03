#!/bin/bash

# Build TailwindCSS
if [ ! -f "dist/tailwind.css" ]; then
  pnpm cross-env NODE_ENV=production postcss src/styles/main.css -o "dist/tailwind.css" --minify
fi

# Copy unhandled assests
cp src/assets/manifest.json dist/manifest.json
cp src/assets/icons/android-chrome-192x192.png dist/android-chrome-192x192.png
cp src/assets/icons/android-chrome-512x512.png dist/android-chrome-512x512.png
cp src/assets/icons/apple-touch-icon.png dist/apple-touch-icon.png
cp src/assets/icons/maskable_icon.png dist/maskable_icon.png
cp src/assets/robots.txt dist/robots.txt
