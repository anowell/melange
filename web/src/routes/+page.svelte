<script lang="ts">
	import { getStats, type TableData } from '$lib/api';
	import ChatDrawer from '$lib/components/ChatDrawer.svelte';
	import PlayerAutocomplete from '$lib/components/PlayerAutocomplete.svelte';
	import TeamAutocomplete from '$lib/components/TeamAutocomplete.svelte';
	import DataTable from '$lib/system/DataTable.svelte';

	let team = $state('');
	let player = $state('');
	let position = $state('');
	let year = $state(2024);
	let weeks = $state('');
	let isLoading = $state(false);

	let data = $state<TableData>();

	// Placeholder for the query function
	async function queryStats() {
		isLoading = true;
		try {
			const params = {
				weeks: weeks || undefined,
				year: year || undefined,
				player: player || undefined,
				position: position || undefined,
				team: team || undefined
			};
			data = await getStats(params);
		} catch (err) {
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="container mx-auto py-4">
	<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-4">
		<label class="form-control">
			<div class="label">
				<span class="label-text">Team</span>
			</div>
			<TeamAutocomplete bind:value={team} class="input-sm" />
		</label>

		<label class="form-control">
			<div class="label">
				<span class="label-text">Position</span>
			</div>
			<select bind:value={position} placeholder="Position" class="select select-sm select-bordered">
				<option value="">Any Position</option>
				{#each ['QB', 'RB', 'WR', 'TE', 'K'] as pos}
					<option value={pos}>{pos}</option>
				{/each}
			</select>
		</label>

		<label class="form-control">
			<div class="label">
				<span class="label-text">Player</span>
			</div>
			<PlayerAutocomplete bind:value={player} class="input-sm" />
		</label>

		<label class="form-control">
			<div class="label">
				<span class="label-text">Year</span>
			</div>
			<select bind:value={year} class="select select-sm select-bordered">
				{#each Array.from({ length: 23 }, (_, i) => 2024 - i) as y}
					<option value={y}>{y}</option>
				{/each}
			</select>
		</label>

		<label class="form-control">
			<div class="label">
				<span class="label-text">Weeks</span>
			</div>
			<input
				type="text"
				bind:value={weeks}
				placeholder="Enter weeks (e.g. 1 or 1-3)"
				class="input input-sm input-bordered"
			/>
		</label>

		<label class="form-control">
			<div class="label">
				<span class="label-text">&nbsp;</span>
			</div>
			<button class="btn btn-primary btn-sm" onclick={queryStats}>Query Stats</button>
		</label>
	</div>

	<div class="divider"></div>

	{#if isLoading}
		<div class="flex justify-center mt-2">
			<span class="loading loading-spinner loading-md"></span>
		</div>
	{:else if data && data.length > 0}
		<div class="w-full overflow-x-auto">
			<DataTable {data} />
		</div>
	{:else if data && data.length === 0}
		<p class="mt-2 text-sm text-error">No results found</p>
	{/if}
</div>

<ChatDrawer />
