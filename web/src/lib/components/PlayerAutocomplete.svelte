<script lang="ts">
	import { searchPlayers } from '$lib/api';
	import toastStore from '$lib/stores/toasts.svelte';
	import Autocomplete from '$lib/system/Autocomplete.svelte';

	let {
		value = $bindable(''),
		onchange,
		class: className
	}: {
		value?: string;
		class?: string;
		onchange?: (value: string) => void;
	} = $props();

	let isLoading = $state(false);
	let completions = $state<string[]>([]);

	async function fetchPlayers(query: string) {
		if (query.length < 2) return;

		isLoading = true;
		try {
			const data = await searchPlayers(query);
			completions = data.map((player) => player.full_name as string);
		} catch (err) {
			const message = `Error fetching player data: ${err}`;
			console.error(message);
			toastStore.addToast(message, 'error');
			completions = [];
		} finally {
			isLoading = false;
		}
	}

	function handleInput(val: string) {
		value = val;
		fetchPlayers(value);
		if (onchange) {
			onchange(value);
		}
	}
</script>

<Autocomplete
	bind:value
	{completions}
	{isLoading}
	placeholder="Search for player (e.g. Geno)"
	oninput={handleInput}
	class={className}
/>
