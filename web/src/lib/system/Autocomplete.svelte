<script lang="ts">
	let {
		completions,
		onchange,
		oninput,
		placeholder,
		isLoading = false,
		class: className,
		value = $bindable('')
	}: {
		value?: string;
		isLoading?: boolean;
		class?: string;
		placeholder?: string;
		completions: Array<string>;
		onchange?: (value: string) => void;
		oninput?: (value: string) => void;
	} = $props();

	let preventBlur = $state(false);

	let suggestions = $derived.by(() => {
		if (!isLoading && value.length > 0) {
			return fuzzySearch(completions, value);
		}
		return [];
	});

	let showSuggestions = $state(false);

	function fuzzySearch(data: Array<string>, query: string) {
		const lowerQuery = query.toLowerCase();
		return data.filter((item) => item.toLowerCase().includes(lowerQuery));
	}

	function selectItem(item: string) {
		value = item;
		showSuggestions = false;
		if (onchange) {
			onchange(value);
		}
	}

	function handleChange() {
		if (onchange) {
			onchange(value);
		}
	}

	function handleInput() {
		showSuggestions = true;
		if (oninput) {
			oninput(value);
		}
	}
</script>

<div class="relative w-full">
	<input
		type="text"
		bind:value
		onblur={() => {
			if (!preventBlur) {
				showSuggestions = false;
			}
		}}
		oninput={handleInput}
		onchange={handleChange}
		{placeholder}
		class={`input input-bordered w-full ${className}`}
	/>

	{#if showSuggestions}
		{#if isLoading}
			<div class="flex justify-center mt-2">
				<span class="loading loading-spinner loading-md"></span>
			</div>
		{:else if suggestions.length}
			<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
			<ul
				class="menu flex-nowrap absolute z-10 bg-base-100 w-full max-h-48 overflow-y-auto shadow-lg rounded-box mt-1"
				onmousedown={() => (preventBlur = true)}
				onmouseup={() => (preventBlur = false)}
			>
				{#each suggestions as suggestion, idx}
					<li>
						<!-- svelte-ignore a11y_no_static_element_interactions -->
						<!-- svelte-ignore a11y_missing_attribute -->
						<!-- svelte-ignore a11y_click_events_have_key_events -->
						<a class="cursor-pointer" onclick={() => selectItem(suggestion)}>
							{suggestion}
						</a>
					</li>
				{/each}
			</ul>
		{:else if value.length >= 2}
			<p class="mt-2 text-sm text-error">No results found</p>
		{/if}
	{/if}
</div>
