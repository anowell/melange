<script lang="ts">
	import toastStore from '$lib/stores/toasts.svelte';

	let toasts = $derived(toastStore.toasts);
</script>

{#if toasts.length > 0}
	<div class="toast">
		{#each toasts as { id, message, type } (id)}
			<div class={`alert alert-${type}`} role="alert">
				<span>{message}</span>
			</div>
		{/each}
	</div>
{/if}
