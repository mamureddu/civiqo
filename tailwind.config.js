/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/server/templates/**/*.html",
  ],
  theme: {
    extend: {
      colors: {
        'civiqo-blue': {
          DEFAULT: '#2563EB',
          dark: '#1D4ED8',
        },
        'civiqo-eco-green': '#57C98A',
        'civiqo-coral': '#FF6B6B',
        'civiqo-lilac': '#A78BFA',
        'civiqo-gray': {
          50: '#F9FAFB',
          100: '#F3F4F6',
          200: '#E5E7EB',
          300: '#D1D5DB',
          400: '#9CA3AF',
          500: '#6B7280',
          600: '#4B5563',
          700: '#374151',
          800: '#1F2937',
          900: '#111827',
        },
      },
      fontFamily: {
        'brand': ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
