<script lang="ts">
	import { Algorithm, selectedAlgorithm} from './store';

	let visible = true;
	function toggleVisible() {
		visible = !visible;
	}

	let currentSelectedAlgorithm: Algorithm = Algorithm.NearestNeighbor;

	function handleChange(event: Event) {
		currentSelectedAlgorithm = (event.target as HTMLInputElement).value as Algorithm;
		selectedAlgorithm.set(currentSelectedAlgorithm);
	}
</script>

<button on:click={toggleVisible} class="done rmenu-category">{visible ? 'Algorithm selection' : 'Show algorithm selection'}</button>

{#if visible}
<div class="algorithm-menu">
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
				value="BruteForce"
				bind:group={currentSelectedAlgorithm}
				on:change={handleChange}
			/>
			Brute Force
		</label>
	</div>
</div>

{/if}

<style>
	label {
		display: block;
		margin-bottom: 0.5rem;
	}

	.done {
		background: hsl(115, 100%, 68%);
	}

	.todo {
		background: hwb(14 30% 0%);
	}
</style>
