<script lang="ts">
	import Toasts from '$lib/system/Toasts.svelte';

	let { children } = $props();
</script>

{@render children()}

<Toasts />
