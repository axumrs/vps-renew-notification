import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "node:path";
import vueSetupExtend from "vite-plugin-vue-setup-extend";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), vueSetupExtend()],
  resolve: {
    alias: {
      "@": path.resolve("./src"),
    },
  },
});