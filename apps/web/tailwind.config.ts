import type { Config } from 'tailwindcss'

const config: Config = {
  darkMode: 'class',
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        surface:   '#0f0f0f',
        'surface-1': '#1a1a1a',
        'surface-2': '#242424',
        primary:   '#6750A4',
      },
    },
  },
  plugins: [],
}
export default config
