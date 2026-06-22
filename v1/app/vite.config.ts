import react from "@vitejs/plugin-react";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const appDir = path.dirname(fileURLToPath(import.meta.url));
const sdkSrc = path.resolve(appDir, "../packages/sdk/src");

export default defineConfig({
  plugins: [react()],
  define: {
    global: "globalThis",
  },
  resolve: {
    alias: {
      "@llm-token-futures/sdk": path.join(sdkSrc, "index.ts"),
      buffer: "buffer/",
    },
  },
  optimizeDeps: {
    include: ["buffer", "@solana/web3.js"],
    exclude: ["@llm-token-futures/sdk"],
  },
  server: {
    port: 5173,
    strictPort: true,
    hmr: {
      clientPort: 5174,
    },
  },
});
