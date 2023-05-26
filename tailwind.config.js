/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {
            colors: {
                "matte-black": "#1F1F1F",
                "pastel-green": "#A2D9B1",
                "pastel-blue": "#B2D8D8",
                "pastel-yellow": "#FFF8A5",
                "pastel-purple": "#D5BEE8",
                "pastel-red": "#FCC9B9",
                "terminal-gray": "#2C2C2C",
                "terminal-dark-blue": "#1E1E2E",
            },
            maxWidth: {
                "header": "1920px",
                "content": "1080px",
            },
            keyframes: {
                shake: {
                    "0%": { transform: "translateX(0%)" },
                    "20%": { transform: "translateX(-2%)" },
                    "40%": { transform: "translateX(2%)" },
                    "60%": { transform: "translateX(-2%)" },
                    "80%": { transform: "translateX(2%)" },
                    "100%": { transform: "translateX(0%)" },
                }
            },
            animation: {
                shake: "shake 1s ease-in-out",
            }
        },
    },
    plugins: [],
}
