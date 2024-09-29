// import adapter from '@sveltejs/adapter-auto';
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			fallback: 'index.html'
		})
	},
	onwarn: (warning, handler) => {
		const { code } = warning;
		if (code.startsWith('a11y')) {
			return false;
		}
		handler(warning);
	},
	vitePlugin: {
		inspector: true
	}
};

export default config;
