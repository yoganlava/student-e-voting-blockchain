import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import { fileURLToPath } from 'url';
import path from 'path';

const __filename = fileURLToPath(import.meta.url);

const __dirname = path.dirname(__filename);

const config: UserConfig = {
	plugins: [sveltekit()],
	resolve: {
		alias: {
			'@terra-money/terra.js': '@terra-money/terra.js/dist/bundle.js',
			process: path.resolve(__dirname, 'src/polyfills/process-es6.js'),
			// 'readable-stream': 'vite-compatible-readable-stream',
			crypto: 'crypto-browserify',
			stream: 'stream-browserify'
			// Buffer: path.resolve(__dirname, 'src/polyfills/Buffer.js')
		}
	},
	define: {
		global: 'globalThis'
	}
};

export default config;
