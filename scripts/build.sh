#!/bin/bash

# Build TailwindCSS
if [ ! -f "apps/web/dist/tailwind.css" ]; then
  pnpm build
fi
