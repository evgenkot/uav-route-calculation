<script lang="ts">
	import Map from 'ol/Map';
	import View from 'ol/View';
	import { OSM, Vector as VectorSource, XYZ } from 'ol/source';
	import { Tile as TileLayer, Vector as VectorLayer } from 'ol/layer';
	import Draw from 'ol/interaction/Draw';
	import Modify from 'ol/interaction/Modify';
	import Snap from 'ol/interaction/Snap';
	import { Circle, Geometry, GeometryCollection, Polygon } from 'ol/geom';
	import { onMount } from 'svelte';

	import { invoke } from '@tauri-apps/api/tauri';

	let viewMap = 'main-map';
	let map: Map;
	let draw: Draw;
	let modify: Modify;
	let snap: Snap;

	onMount(async () => {
		const osmLayer = new TileLayer({ source: new OSM() });

		const source = new VectorSource({ wrapX: false });

		const vector = new VectorLayer({
			source: source
		});

		map = new Map({
			target: viewMap,
			layers: [osmLayer, vector],
			view: new View({
				center: [0, 0],
				zoom: 2
			})
		});

		draw = new Draw({
			source: source,
			type: 'Polygon'
		});
		// map.addInteraction(draw);

		modify = new Modify({ source: source });
		map.addInteraction(modify);

		snap = new Snap({ source: source });
		map.addInteraction(snap);
	});

	function undo() {
		draw.removeLastPoint();
	}

	function removeDeleteVertex() {
		modify.unset('deleteCondition');
	}

	function enableNavigation() {
		map.removeInteraction(draw);
		map.removeInteraction(modify);
		map.removeInteraction(snap);
	}

	function enableDrawing() {
		map.addInteraction(draw);
		map.addInteraction(modify);
		map.addInteraction(snap);
	}

	async function displayVertices() {
		const vectorLayer = map
			.getLayers()
			.getArray()
			.find((layer) => layer instanceof VectorLayer) as VectorLayer;
		const source = vectorLayer.getSource() as VectorSource;
		const features = source.getFeatures();
		const vertices = features
			.filter((feature) => feature.getGeometry() instanceof Polygon)
			.map((feature) => feature.getGeometry().getCoordinates()[0]);

		// Send the coordinates to the Rust backend
		await invoke('receive_polygon_coordinates', { vertices });
	}
</script>

<div id={viewMap} class="map" />
<div class="toolbar">
	<button on:click={undo}>Undo</button>
	<button on:click={enableDrawing}>Draw</button>
	<button on:click={enableNavigation}>Navigation</button>
	<button on:click={displayVertices}>Display Vertices</button>
</div>

<style>
	@import '../styles/map-style.css';
	@import '../global.css';
	.toolbar {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 50px;
		display: flex;
		align-items: center;
		justify-content: space-evenly;
		pointer-events: none;
	}
	.toolbar button {
		pointer-events: auto;
		z-index: 1;
	}
	.map {
		margin: 0;
		padding: 0;
		width: 100%;
		height: 100%;
	}
</style>
