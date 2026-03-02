import { defineConfig, presetUno, presetAttributify, presetIcons } from 'unocss'
import transformerDirectives from '@unocss/transformer-directives'

export default defineConfig({
    presets: [
        presetUno(),
        presetAttributify(),
        presetIcons({
            scale: 1.2,
            warn: true,
            prefix: 'i-',
            extraProperties: {
                'display': 'inline-block',
                'vertical-align': 'middle',
            },
        }),
    ],
    transformers: [transformerDirectives()],
    content: {
        pipeline: {
            include: [/\.tsx$/],
        },
    },
    safelist: [
        // TitleBar
        'i-carbon-moon', 'i-carbon-sun', 'i-carbon-pin', 'i-carbon-subtract',
        'i-carbon-maximize', 'i-carbon-fit-to-screen', 'i-carbon-close',
        'i-carbon-information',
        // NavBar / 页面
        'i-carbon-home', 'i-carbon-search', 'i-carbon-download',
        'i-carbon-settings', 'i-carbon-star-filled', 'i-carbon-arrow-right',
        'i-carbon-category', 'i-carbon-application', 'i-carbon-update-now',
        'i-carbon-checkmark-filled', 'i-carbon-warning', 'i-carbon-cloud-download',
        'i-carbon-launch', 'i-carbon-overflow-menu-vertical', 'i-carbon-arrow-up',
    ],
    theme: {
        colors: {
            brand: {
                DEFAULT: '#1296DB',
                blue: '#1296DB',
                cyan: '#80D8FF',
                deep: '#0d7ab5',
                light: '#80D8FF',
            },
            dark: {
                900: '#08080c',
                800: '#0c0c12',
                700: '#101018',
                600: '#16161f',
                500: '#1e1e2a',
            },
        },
    },
    shortcuts: {
        'glass-card': 'bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl',
        'flex-center': 'flex items-center justify-center',
        'flex-between': 'flex items-center justify-between',
        'btn': 'px-4 py-2 rounded-xl font-medium transition-all duration-300 cursor-pointer select-none',
        'btn-primary': 'btn bg-gradient-to-r from-brand-blue to-brand-cyan text-white',
        'btn-ghost': 'btn text-white/50 hover:text-white hover:bg-white/5',
    },
})
