import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import pugPreprocessor from "./vite/pug/build.js";
import { stylusVitePlugin, svelteStylusPreprocessor } from "./vite/stylus.js";
import { htmlMinifyPlugin } from "./vite/minify.js";
import { browserslistToTargets } from "lightningcss";
import browserslist from "browserslist";

export default defineConfig(({ mode }) => {
  const is_prod = mode === "production";

  return {
    base: "./",
    plugins: [
      stylusVitePlugin(),
      svelte({
        preprocess: [pugPreprocessor(), svelteStylusPreprocessor()],
      }),
      is_prod ? htmlMinifyPlugin() : null,
    ].filter(Boolean),
    css: {
      lightningcss: {
        targets: browserslistToTargets(browserslist("> 0.25%, not dead")),
      },
    },
    build: {
      target: "esnext",
      minify: is_prod ? "oxc" : false,
      cssMinify: is_prod ? "lightningcss" : false,
      outDir: "dist",
      emptyOutDir: true,
      assetsInlineLimit: 4096,
    },
    server: {
      port: 5173,
      host: true,
    },
  };
});
