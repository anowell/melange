<script lang="ts">
	import Octocat from '$lib/system/Octocat.svelte';
	import Toasts from '$lib/system/Toasts.svelte';

	let { children } = $props();
</script>

<div class="min-h-screen flex flex-col">
	<div class="flex-grow">
		{@render children()}
	</div>

	<footer class="footer footer-center bg-base-300 text-base-content p-4">
		<aside>
			<p>Built with <a href="https://spice.ai" class="link">Spice</a></p>
		</aside>
	</footer>
</div>

<Octocat />
<Toasts />
