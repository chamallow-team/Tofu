const defaultTheme = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
module.exports = {
    purge: ['./index.html', './public/index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
    theme: {
        extend: {
            colors: {
                'dark': '#0e0e11',
                'cream': '#e2c2b2',
                'onyx': '#353641',
                'discord': '#5865f2',
                'greenish': '#2a4c4a',
                'sheen': "#63b2ae",
                'chrome-yellow': '#ffa800'
            },
            fontFamily: {
                sans: ['Poppins', ...defaultTheme.fontFamily.sans]
            }
        },
    },
    plugins: [],
}

