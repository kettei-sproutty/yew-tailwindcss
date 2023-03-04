# Yew and Tailwind CSS Scaffold

This scaffold provides a basic setup for a Yew app with Tailwind CSS for styling. It also includes `DaisyUI`, a Tailwind CSS component library, and uses `xtask-wasm` as a development server and build tool. Additionally, it uses PNPM as a package manager and PostCSS to process CSS files.

## To-Do

-   [x] Set up GitHub Actions for format and linting
-   [x] Handle deployment on Vercel without removing `dist` from `.gitignore`
-   [ ] Add PWA
-   [ ] Add authentication
-   [ ] Create protected routes
-   [ ] Add server-side rendering (SSR)

## Technologies

## Technologies

-   [Yew](https://yew.rs/) - A modern Rust framework for building web applications
-   [Tailwind CSS](https://tailwindcss.com/) - A utility-first CSS framework
-   [DaisyUI](https://daisyui.com/) - A Tailwind CSS component library
-   [xtask](https://github.com/matklad/cargo-xtask) - A task runner for Rust projects
-   [turborepo](https://turborepo.com/) - A tool for managing monorepos
-   [cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) - A feature of Cargo for managing multiple Rust packages
-   [PNPM](https://pnpm.js.org/) - A fast, disk space efficient package manager for JavaScript
-   [PostCSS](https://postcss.org/) - A CSS preprocessor

## Installation

To get started, clone this repository and install the required dependencies:

```sh
git clone https://github.com/kettei-sproutty/yew-tailwindcss.git
cd yew-tailwindcss
rustup target add wasm32-unknown-unknown
pnpm install
```

This will install all the required dependencies.

## Development

To start the development server, run:

```sh
cargo xtask start
```

This will start the development server at `http://localhost:8000`, with live reloading for any changes you make to the Rust or CSS files.

To compile the app for production, run:

```sh
cargo xtask dist
```

This will compile the app in release mode and optimize it for performance.

CSS is generated using `postcss`.

That's it! You now have a basic Yew app with Tailwind CSS and DaisyUI for styling. From here, you can customize the app to fit your needs and add more components as necessary.

## Trunk

Trunk configuration can still be seen at the [trunk branch](https://github.com/kettei-sproutty/yew-tailwindcss/tree/trunk) branch.
