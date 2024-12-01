import {resolve} from 'path';
import {defineConfig} from "vite";
import vue from "@vitejs/plugin-vue";
import UnoCSS from 'unocss/vite'
import presetUno from '@unocss/preset-uno';
import {createSvgIconsPlugin} from 'vite-plugin-svg-icons';

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    plugins: [
        vue(),
        UnoCSS({
            presets: [presetUno()],
        }),
        createSvgIconsPlugin({
            iconDirs: [
                resolve(__dirname, './src/assets/icons/svg'),
            ],
            symbolId: 'icon-[dir]-[name]',
        }),
    ],
    resolve: {
        alias: {
            "@": resolve(__dirname, "./src"),
        },
    },
    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    build: {
        chunkSizeWarningLimit: 4096,
    },
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                protocol: "ws",
                host,
                port: 1421,
            }
            : undefined,
        watch: {
            // 3. tell vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"],
        },
    },
}));
