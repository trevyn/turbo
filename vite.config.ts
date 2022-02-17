import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { viteSingleFile } from "vite-plugin-singlefile";

// https://vitejs.dev/config/
export default defineConfig({
 root: "src-frontend",
 plugins: [svelte(), viteSingleFile()],
 build: {
  target: "es2021",
  assetsInlineLimit: 100000000,
  chunkSizeWarningLimit: 100000000,
  cssCodeSplit: false,
  brotliSize: false,
  rollupOptions: {
   inlineDynamicImports: true,
   output: {
    manualChunks: () => "everything.js",
   },
  },
 },
});
