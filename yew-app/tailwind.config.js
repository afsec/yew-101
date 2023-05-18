/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html,rs}",
    "./index.html"
  ],
  daisyui: {
    themes: ["coffee"],
  },
  theme: {
    extend: {},
  },
  plugins: [
    require("@tailwindcss/typography"),
    require("daisyui")
  ],
}