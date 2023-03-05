<script>
	import Draw from 'ol/interaction/Draw.js';
	import Map from 'ol/Map.js';
	import View from 'ol/View.js';
	import { OSM, Vector as VectorSource } from 'ol/source.js';
	import { Tile as TileLayer, Vector as VectorLayer } from 'ol/layer.js';

	export let viewMap;

	const typeSelect = document.getElementById('type');

	let draw; // global so we can remove it later
	function addInteraction() {
		const value = typeSelect.value;
		if (value !== 'None') {
			draw = new Draw({
				source: source,
				type: typeSelect.value
			});
			map.addInteraction(draw);
		}
	}

	/**
	 * Handle change event.
	 */
	typeSelect.onchange = function () {
		map.removeInteraction(draw);
		addInteraction();
	};

	document.getElementById('undo').addEventListener('click', function () {
		draw.removeLastPoint();
	});

	addInteraction();
</script>

<div class="row">
	<div class="col-auto">
		<span class="input-group">
			<label class="input-group-text" for="type">Geometry type:</label>
			<select class="form-select" id="type">
				<option value="Point">Point</option>
				<option value="LineString">LineString</option>
				<option value="Polygon">Polygon</option>
				<option value="Circle">Circle</option>
				<option value="None">None</option>
			</select>
			<input class="form-control" type="button" value="Undo" id="undo" />
		</span>
	</div>
</div>

<style>
</style>
