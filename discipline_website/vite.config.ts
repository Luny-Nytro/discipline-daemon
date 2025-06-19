import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import * as Path from "node:path"
import * as Url from "node:url"

const packageDirectoryPath = Path.dirname(Url.fileURLToPath(import.meta.url))

export default defineConfig({
	plugins: [
		tailwindcss(), 
		sveltekit(),
	],
	resolve: {
		alias: {
			"Discipline":  Path.join(packageDirectoryPath, "../DisciplineClientTs/CompiledCode/Public.js"),
			"DisciplineDir/":  Path.join(packageDirectoryPath, "../DisciplineClientTs/CompiledCode/"),
		},
	}
});
