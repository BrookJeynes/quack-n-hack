/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {
            colors: {
                "matte-black": "#1F1F1F",
                "mint-green": "#B0CD82",
                "terminal-gray": "#2C2C2C",
            },
            maxWidth: {
                "header": "1920px",
                "content": "1080px",
            }
        },
    },
    plugins: [],
}
