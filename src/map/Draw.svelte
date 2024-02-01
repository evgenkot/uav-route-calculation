<script lang="ts">
	import {
		map,
		drawInteraction,
		modifyInteraction,
		snapInteraction,
		vectorPolySource,
		areaSelected
	} from './store';
	import { Algorithm } from './store';

	let isDrawing = false;

	let visible = true;
	function toggleVisible() {
		visible = !visible;
	}

	function toggleDrawing() {
		if (isDrawing) {
			isDrawing = !isDrawing;
			$map.removeInteraction($drawInteraction);
			$map.removeInteraction($modifyInteraction);
			$map.removeInteraction($snapInteraction);
			console.log('You are now not Drawing');
		} else {
			isDrawing = !isDrawing;
			$map.addInteraction($drawInteraction);
			$map.addInteraction($modifyInteraction);
			$map.addInteraction($snapInteraction);
			console.log('You are now Drawing');
			$areaSelected = false;
		}
	}

	function undoPolygon() {
		if ($vectorPolySource.getFeatures().length > 0) {
			$vectorPolySource.removeFeature(
				$vectorPolySource.getFeatures()[$vectorPolySource.getFeatures().length - 1]
			);
		}
		$areaSelected = false;
	}

	function undoPoint() {
		$drawInteraction.removeLastPoint();
		$areaSelected = false;
	}

	function checkPolygon() {
	
		if ($vectorPolySource.getFeatures().length != 0) {
			$areaSelected = true;
		} else {
			$areaSelected = false;
		}
	}
</script>

<!-- class="toggle-display" -->
<button on:click={toggleVisible} class="{$areaSelected ? 'done' : 'todo'} rmenu-category">{visible ? 'Area selection' : 'Show area selection'}</button>
{#if visible}
	<div class="block" id="drawBlock">
		<div class="draw-mode-label">Draw Mode</div>
		<button on:click={toggleDrawing}>{isDrawing ? 'Stop' : 'Start'} Drawing</button>
		<button on:click={undoPolygon}>Undo Polygon</button>
		<button on:click={undoPoint}>Undo Point</button>
		<button on:click={checkPolygon}>Check</button>
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
