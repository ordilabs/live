/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/aspect-ratio'),
  ]
}
