/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	safelist: ['alert-success', 'alert-error', 'alert-info', 'alert-warning'],
	theme: {
		extend: {}
	},
	// eslint-disable-next-line @typescript-eslint/no-require-imports
	plugins: [require('@tailwindcss/typography'), require('daisyui')],
	daisyui: {
		logs: false,
		themes: ['fantasy', 'dracula'],
		darkTheme: 'dracula'
	}
};
