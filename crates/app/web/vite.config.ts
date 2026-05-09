import { defineConfig } from "vite";
import { resolve } from "node:path";

export default defineConfig({
	root: resolve(__dirname),
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
		host: "127.0.0.1"
	},
	build: {
		outDir: "dist",
		emptyOutDir: true
	}
});