import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [sveltekit()],
	test: {
		include: ['tests/**/*.{test,spec}.{js,ts}']
	},
	server: {
		fs: {
			allow: ['app']
		},
		proxy: {
			'/api/': {
				target: 'http://127.0.0.1:3000',
				changeOrigin: true,
			}
		}
	}
});
