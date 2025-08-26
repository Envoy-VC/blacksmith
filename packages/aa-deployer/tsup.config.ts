import { defineConfig } from "tsup";

export default defineConfig({
  clean: true,
  dts: true,
  entry: ["src/index.ts"],
  format: ["esm", "cjs"],
  sourcemap: "inline",
  splitting: true,
  treeshake: true,
  tsconfig: "tsconfig.build.json",
});
