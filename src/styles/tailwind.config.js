/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
    theme: {
        fontFamily: {
            sans: ["Inter", "sans-serif"],
        },
        extend: {},
    },
    plugins: [],
};
