<script lang="ts">
	import { writable } from 'svelte/store';
	import { altitudeValue, selectedCamera, overlapValue } from './store';

	let altitudeMode = 'manual';
	const smPerValue = writable<number>(0);

	function onAltitudeModeChange(event: Event) {
		altitudeMode = (event.target as HTMLInputElement).value;
	}

	function onAltitudeValueChange(event: Event) {
		altitudeValue.set(parseFloat((event.target as HTMLInputElement).value));
	}

	function onSmPerPxValueChange(event: Event) {
		smPerValue.set(parseFloat((event.target as HTMLInputElement).value));
	}

	function calculateAltitude() {
		smPerValue.update((value) => {
			if ($selectedCamera) {
				const overlap = $overlapValue / 100;
				const tg = (angle: number) => Math.tan(angle * (Math.PI / 180));
				const calculatedAltitude =
					($selectedCamera.resolution_x * (1 - overlap) * 0.5 * value) /
					(tg($selectedCamera.fov_x * 0.5) * 100);
				altitudeValue.set(calculatedAltitude);
			}
			return value;
		});
	}
</script>

<div class="altitude-menu">
	<h2>Altitude</h2>
	<div class="input-row">
		<input
			type="radio"
			id="manual-altitude"
			name="altitude-mode"
			value="manual"
			checked={altitudeMode === 'manual'}
			on:change={onAltitudeModeChange}
		/>
		<label for="manual-altitude">Manual altitude input</label>
	</div>
	<div class="input-row">
		<input
			type="radio"
			id="calculate-sm-px"
			name="altitude-mode"
			value="calculate"
			checked={altitudeMode === 'calculate'}
			on:change={onAltitudeModeChange}
		/>
		<label for="calculate-sm-px">Calculate using sm/px</label>
	</div>
	<div class="input-row">
		<label for="overlap-percentage">Overlap (%):</label>
		<input
			type="number"
			id="overlap-percentage"
			min="0"
			max="100"
			step="0.1"
			bind:value={$overlapValue}
		/>
	</div>
	<div class="input-row">
		<label for="altitude">Altitude:</label>
		<input
			type="number"
			id="altitude"
			min="0"
			step="0.001"
			disabled={altitudeMode === 'calculate'}
			on:input={onAltitudeValueChange}
			bind:value={$altitudeValue}
		/>
	</div>
	{#if altitudeMode === 'calculate'}
		<div class="input-row">
			<label for="sm-per-px">sm/px:</label>
			<input type="number" id="sm-per-px" min="0" step="0.01" on:input={onSmPerPxValueChange} />
		</div>
		<div class="input-row">
			<button on:click={calculateAltitude}>Calculate Altitude</button>
		</div>
	{/if}
</div>

<style>
	
</style>
