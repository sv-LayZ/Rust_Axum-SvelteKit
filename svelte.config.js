import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: 'app.html',
			precompress: false,
			strict: true
		}),
		// prerender: {
		// 	entries: ['*']
		// },
		files: {
			lib: 'app/lib',
			routes: 'app/views',
			appTemplate: 'app/app.html',
			errorTemplate: 'app/error.html'
		}
	}
};

export default config;
