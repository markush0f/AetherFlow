/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'brand-orange': '#FF6B00',
        'brand-blue': '#00A3FF',
        'brand-black': '#0A0A0A',
      }
    },
  },
  plugins: [],
}

