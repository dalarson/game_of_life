import { defineConfig } from "vite";

export default defineConfig({
  server: {
    open: true,
  },
  // Ensure .wasm files in pkg/ are served correctly
  assetsInclude: ["**/*.wasm"],
});
