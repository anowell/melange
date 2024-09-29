<script lang="ts">
	import Autocomplete from '$lib/system/Autocomplete.svelte';

	let {
		value = $bindable(''),
		onchange,
		class: className
	}: {
		value?: string;
		onchange?: (value: string) => void;
		class?: string;
	} = $props();

	// List of NFL teams with full name and abbreviation
	const teams = [
		{ name: 'Arizona Cardinals', abbr: 'ARI' },
		{ name: 'Atlanta Falcons', abbr: 'ATL' },
		{ name: 'Baltimore Ravens', abbr: 'BAL' },
		{ name: 'Buffalo Bills', abbr: 'BUF' },
		{ name: 'Carolina Panthers', abbr: 'CAR' },
		{ name: 'Chicago Bears', abbr: 'CHI' },
		{ name: 'Cincinnati Bengals', abbr: 'CIN' },
		{ name: 'Cleveland Browns', abbr: 'CLE' },
		{ name: 'Dallas Cowboys', abbr: 'DAL' },
		{ name: 'Denver Broncos', abbr: 'DEN' },
		{ name: 'Detroit Lions', abbr: 'DET' },
		{ name: 'Green Bay Packers', abbr: 'GB' },
		{ name: 'Houston Texans', abbr: 'HOU' },
		{ name: 'Indianapolis Colts', abbr: 'IND' },
		{ name: 'Jacksonville Jaguars', abbr: 'JAX' },
		{ name: 'Kansas City Chiefs', abbr: 'KC' },
		{ name: 'Las Vegas Raiders', abbr: 'LV' },
		{ name: 'Los Angeles Chargers', abbr: 'LAC' },
		{ name: 'Los Angeles Rams', abbr: 'LAR' },
		{ name: 'Miami Dolphins', abbr: 'MIA' },
		{ name: 'Minnesota Vikings', abbr: 'MIN' },
		{ name: 'New England Patriots', abbr: 'NE' },
		{ name: 'New Orleans Saints', abbr: 'NO' },
		{ name: 'New York Giants', abbr: 'NYG' },
		{ name: 'New York Jets', abbr: 'NYJ' },
		{ name: 'Philadelphia Eagles', abbr: 'PHI' },
		{ name: 'Pittsburgh Steelers', abbr: 'PIT' },
		{ name: 'San Francisco 49ers', abbr: 'SF' },
		{ name: 'Seattle Seahawks', abbr: 'SEA' },
		{ name: 'Tampa Bay Buccaneers', abbr: 'TB' },
		{ name: 'Tennessee Titans', abbr: 'TEN' },
		{ name: 'Washington Commanders', abbr: 'WAS' }
	];

	const allSuggestions = $derived(teams.map((team) => team.name));

	function mapToAbbreviation(name: string) {
		const match = teams.find((team) => name === team.name);
		return match ? match.abbr : '';
	}

	function handleChange(val: string) {
		value = mapToAbbreviation(val);
		if (onchange) {
			onchange(value);
		}
	}
</script>

<Autocomplete
	completions={allSuggestions}
	placeholder="Enter team (e.g. Seahawks)"
	onchange={handleChange}
	class={className}
/>
