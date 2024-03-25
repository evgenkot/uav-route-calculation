<script lang="ts">
	import { writable } from 'svelte/store';
	import {
		altitudeValue,
		selectedCamera,
		overlapValue,
		altitudeSelected,
		selectedUav
	} from './store';

	let visible = true;
	function toggleVisible() {
		visible = !visible;
	}

	function checkAltitude() {
		if (!$selectedUav) {
			alert('Uav not set');
			$altitudeSelected = false;
		} else if (
			$selectedUav.min_altitude > $altitudeValue ||
			$selectedUav.max_altitude < $altitudeValue
		) {
			alert('Altitude out of range');
			$altitudeSelected = false;
		} else {
			$altitudeSelected = true;
		}
	}

	let altitudeMode = 'manual';
	const smPerValue = writable<number>(0);

	function onAltitudeModeChange(event: Event) {
		$altitudeSelected = false;
		altitudeMode = (event.target as HTMLInputElement).value;
	}

	function onAltitudeValueChange(event: Event) {
		$altitudeSelected = false;
		altitudeValue.set(parseFloat((event.target as HTMLInputElement).value));
	}

	function onSmPerPxValueChange(event: Event) {
		$altitudeSelected = false;
		smPerValue.set(parseFloat((event.target as HTMLInputElement).value));
	}

	function calculateAltitude() {
		$altitudeSelected = false;
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

<button on:click={toggleVisible} class="{$altitudeSelected ? 'done' : 'todo'} rmenu-category"
	>{visible ? 'Altitude selection' : 'Show altitude selection'}</button
><br />

{#if visible}
	<div class="altitude-menu">
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

		<button on:click={checkAltitude}>Check</button>
	</div>
{/if}

<style>
	.done {
		background: hsl(115, 100%, 68%);
	}

	.todo {
		background: hwb(14 30% 0%);
	}
</style>
