import adapter from '@sveltejs/adapter-auto';

export default {
  kit: {
    // hydrate the <div id="svelte"> element in src/app.html
    target: '#svelte',
    adapter: adapter()
  }
};