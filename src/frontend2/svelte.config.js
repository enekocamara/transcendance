import adapter from '@sveltejs/adapter-auto';
import preprocess from 'svelte-preprocess';
import { readFileSync } from 'fs';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: preprocess(),
	kit: {
		adapter: adapter(),
	}
};

export default config;
