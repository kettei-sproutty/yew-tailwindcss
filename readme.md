# Yew and Tailwind CSS Scaffold

This scaffold provides a basic setup for a Yew app with Tailwind CSS for styling. It also includes DaisyUI, a Tailwind CSS component library, and uses Trunk as a development server and build tool. Additionally, it uses PNPM as a package manager and PostCSS to process CSS files.

## To-Do

-   [ ] Set up GitHub Actions for format and linting
-   [x] Handle deployment on Vercel without removing `dist` from `.gitignore`
-   [ ] Improve building time
-   [ ] Add server-side rendering (SSR)

## Technologies

-   [Yew](https://yew.rs/) - A modern Rust framework for building web applications
-   [Tailwind CSS](https://tailwindcss.com/) - A utility-first CSS framework
-   [DaisyUI](https://daisyui.com/) - A Tailwind CSS component library
-   [Trunk](https://trunkrs.dev/) - A build tool and development server for Rust web applications
-   [PNPM](https://pnpm.js.org/) - A fast, disk space efficient package manager for JavaScript
-   [PostCSS](https://postcss.org/) - A CSS preprocessor

## Installation

To get started, clone this repository and install the required dependencies:

```sh
git clone https://github.com/kettei-sproutty/yew-tailwindcss.git
cd yew-tailwindcss
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
pnpm install
```

This will install all the required dependencies.

## Development

To start the development server, run:

```sh
trunk serve
```

This will start the development server at `http://localhost:8080`, with live reloading for any changes you make to the Rust or CSS files.

To compile the app for production, run:

```sh
trunk build --release
```

This will compile the app in release mode and optimize it for performance.

CSS is generated using `pnpm` and `trunk build hook`.

That's it! You now have a basic Yew app with Tailwind CSS and DaisyUI for styling. From here, you can customize the app to fit your needs and add more components as necessary.
