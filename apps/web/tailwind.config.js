/**
 * @type {import('tailwindcss/defaultConfig')}
 */
module.exports = {
    content: [".pages/**/*.rs", "./components/**/*.rs", "./public/**/*.html"],
    theme: {
        extend: {},
    },
    variants: {
        extend: {},
    },
    plugins: [require("daisyui")],
};
