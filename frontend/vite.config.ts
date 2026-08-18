import { execSync } from "node:child_process";
import { fileURLToPath, URL } from "node:url";

import babel from "@rolldown/plugin-babel";
import tailwindcss from "@tailwindcss/vite";
import react, { reactCompilerPreset } from "@vitejs/plugin-react";
import { defineConfig, type Plugin } from "vite";

const configDir = fileURLToPath(new URL(".", import.meta.url));
const specPath = fileURLToPath(new URL("../openapi.json", import.meta.url));

function openApiTypesPlugin(): Plugin {
  return {
    name: "openapi-types",
    buildStart() {
      execSync("pnpm api:gen", { cwd: configDir, stdio: "inherit" });
      this.addWatchFile(specPath);
    },
    watchChange(id) {
      if (id === specPath) {
        execSync("pnpm api:gen", { cwd: configDir, stdio: "inherit" });
      }
    },
  };
}

export default defineConfig({
  plugins: [
    openApiTypesPlugin(),
    react(),
    babel({
      presets: [reactCompilerPreset()],
    }),
    tailwindcss(),
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  server: {
    proxy: {
      "/api": "http://localhost:3000",
      "/auth": "http://localhost:3000",
    },
  },
});
