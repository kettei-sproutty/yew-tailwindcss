#!/bin/bash

# Build TailwindCSS
if [ ! -f "dist/tailwind.css" ]; then
  pnpm cross-env NODE_ENV=production postcss src/styles/main.css -o "dist/tailwind.css" --minify
fi
