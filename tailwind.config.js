/**
 * @type {import('tailwindcss/defaultConfig')}
 */
module.exports = {
    content: ["./src/**/*.rs", "./static/**/*.html"],
    theme: {
        extend: {},
    },
    variants: {
        extend: {},
    },
    plugins: [require("daisyui")],
};
