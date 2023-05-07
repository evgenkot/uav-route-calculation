<script lang="ts">
	import { writable } from 'svelte/store';
	import { Algorithm, selectedAlgorithm } from './store';

	let currentSelectedAlgorithm: Algorithm = Algorithm.NearestNeighbor;

	function handleChange(event: Event) {
		currentSelectedAlgorithm = (event.target as HTMLInputElement).value as Algorithm;
		selectedAlgorithm.set(currentSelectedAlgorithm);
	}
</script>

<div class="algorithm-menu">
	<h2>Algorithm</h2>
	<div>
		<label>
			<input
				type="radio"
				name="algorithm"
				value="NearestNeighbor"
				bind:group={currentSelectedAlgorithm}
				on:change={handleChange}
				checked
			/>
			Nearest Neighbor
		</label>
	</div>
	<div>
		<label>
			<input
				type="radio"
				name="algorithm"
				value="ChristofidesAlgorithm"
				bind:group={currentSelectedAlgorithm}
				on:change={handleChange}
			/>
			Christofides Algorithm
		</label>
	</div>
	<div>
		<label>
			<input
				type="radio"
				name="algorithm"
				value="BruteForce"
				bind:group={currentSelectedAlgorithm}
				on:change={handleChange}
			/>
			Brute Force
		</label>
	</div>
</div>

<style>
	label {
		display: block;
		margin-bottom: 0.5rem;
	}
</style>
