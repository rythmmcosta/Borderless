import type { Config } from 'tailwindcss'
import typography from '@tailwindcss/typography'

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
        surface:     '#0e0e14',
        'surface-1': '#141420',
        'surface-2': '#1e1e2e',
        'surface-3': '#252535',
        primary:     '#7c3aed',
        'primary-light': '#a78bfa',
        accent:      '#22d3ee',
      },
      backgroundImage: {
        'gradient-radial':    'radial-gradient(var(--tw-gradient-stops))',
        'gradient-mesh':      'radial-gradient(ellipse 80% 50% at 50% -20%, rgba(124,58,237,0.15), transparent)',
        'gradient-mesh-cyan': 'radial-gradient(ellipse 60% 40% at 80% 80%, rgba(34,211,238,0.08), transparent)',
        'glow-primary':       'radial-gradient(circle at center, rgba(124,58,237,0.4), transparent 70%)',
      },
      animation: {
        'fade-up':    'fadeUp 0.6s ease-out both',
        'fade-in':    'fadeIn 0.5s ease-out both',
        'pulse-slow': 'pulse 4s ease-in-out infinite',
        'float':      'float 6s ease-in-out infinite',
        'glow':       'glow 2s ease-in-out infinite alternate',
        'slide-right':'slideRight 0.3s ease-out',
      },
      keyframes: {
        fadeUp: {
          from: { opacity: '0', transform: 'translateY(20px)' },
          to:   { opacity: '1', transform: 'translateY(0)' },
        },
        fadeIn: {
          from: { opacity: '0' },
          to:   { opacity: '1' },
        },
        float: {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%':      { transform: 'translateY(-10px)' },
        },
        glow: {
          from: { boxShadow: '0 0 20px rgba(124,58,237,0.3)' },
          to:   { boxShadow: '0 0 40px rgba(124,58,237,0.6)' },
        },
        slideRight: {
          from: { transform: 'translateX(-10px)', opacity: '0' },
          to:   { transform: 'translateX(0)', opacity: '1' },
        },
      },
      boxShadow: {
        'glow-sm':  '0 0 20px rgba(124,58,237,0.25)',
        'glow-md':  '0 0 40px rgba(124,58,237,0.35)',
        'glow-lg':  '0 0 80px rgba(124,58,237,0.3)',
        'glow-cyan':'0 0 30px rgba(34,211,238,0.2)',
        'card':     '0 1px 3px rgba(0,0,0,0.5), 0 0 0 1px rgba(255,255,255,0.05)',
        'card-hover':'0 4px 20px rgba(0,0,0,0.5), 0 0 0 1px rgba(124,58,237,0.3)',
      },
      borderColor: {
        subtle:  'rgba(255,255,255,0.06)',
        default: 'rgba(255,255,255,0.10)',
        strong:  'rgba(255,255,255,0.18)',
      },
    },
  },
  plugins: [typography],
}
export default config
