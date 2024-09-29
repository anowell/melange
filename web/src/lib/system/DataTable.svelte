<script lang="ts">
	import { type TableData } from '$lib/api';

	let { data } = $props<{ data: TableData }>();
	let sortBy = $state<string | null>(null);
	let sortOrder = $state<'asc' | 'desc' | null>(null);

	// Extract table headers from the keys of the first record
	let headers = $derived.by(() => Object.keys(data[0] || {}));

	// Sort the data based on the current sort state
	let sortedData = $derived.by(() => {
		if (sortBy && sortOrder) {
			return [...data].sort((a, b) => {
				// asserting non-null because of the check outside this closure
				const aValue = a[sortBy!];
				const bValue = b[sortBy!];

				// Handle null or undefined values, considering them the smallest
				if (aValue == null && bValue != null) return sortOrder === 'asc' ? -1 : 1;
				if (aValue != null && bValue == null) return sortOrder === 'asc' ? 1 : -1;
				if (aValue == null && bValue == null) return 0;

				if (aValue < bValue) return sortOrder === 'asc' ? -1 : 1;
				if (aValue > bValue) return sortOrder === 'asc' ? 1 : -1;
				return 0;
			});
		}
		return data;
	});

	function toggleSort(header: string) {
		if (sortBy === header) {
			sortOrder = sortOrder === 'asc' ? 'desc' : 'asc';
		} else {
			sortBy = header;
			sortOrder = 'desc';
		}
	}
</script>

<table class="table table-compact table-xs w-full">
	<thead>
		<tr>
			{#each headers as header}
				<th onclick={() => toggleSort(header)} class="cursor-pointer">
					{header}
					{#if sortBy === header}
						{#if sortOrder === 'asc'}
							▲
						{/if}
						{#if sortOrder === 'desc'}
							▼
						{/if}
					{/if}
				</th>
			{/each}
		</tr>
	</thead>
	<tbody class="text-nowrap">
		{#each sortedData as row}
			<tr>
				{#each headers as header}
					<td>{row[header]}</td>
				{/each}
			</tr>
		{/each}
	</tbody>
</table>
