import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'
import UnoCSS from 'unocss/vite'
import { resolve } from 'path'

/// 共享包路径
const saUiPath = resolve(__dirname, '../packages/sa-ui')

export default defineConfig(({ mode }) => ({
    plugins: [
        solid(),
        UnoCSS({ inspector: mode === 'development' }),
    ],
    resolve: {
        alias: {
            '@': resolve(__dirname, 'src'),
            '@sa/ui': resolve(saUiPath, 'src'),
            '@sa-styles': resolve(saUiPath, 'src'),
        },
        dedupe: ['solid-js', '@tauri-apps/api'],
    },
    clearScreen: false,
    server: {
        port: 5180,
        strictPort: true,
    },
    envPrefix: ['VITE_', 'TAURI_'],
    build: {
        target: ['es2021', 'chrome100', 'safari15'],
        minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
        sourcemap: !!process.env.TAURI_DEBUG,
        rollupOptions: {
            checks: { pluginTimings: false },
        },
    },
}))
