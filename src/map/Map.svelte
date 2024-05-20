<script lang="ts">
	//OL imports
	import Map from 'ol/Map';
	import View from 'ol/View';
	import { OSM, Vector as VectorSource } from 'ol/source';
	import { Tile as TileLayer, Vector as VectorLayer } from 'ol/layer';
	import Draw from 'ol/interaction/Draw';
	import Modify from 'ol/interaction/Modify';
	import Snap from 'ol/interaction/Snap';
	import { onMount } from 'svelte';
	import Style from 'ol/style/Style';
	import Circle from 'ol/style/Circle';
	import Fill from 'ol/style/Fill';
	import Stroke from 'ol/style/Stroke';

	// Import Svelte stores for accessing reactive values
	import {
		map,
		drawInteraction,
		modifyInteraction,
		snapInteraction,
		startPointSource,
		vectorPolySource,
		discretizedAreaLayer,
		planLayer
	} from './store';

	// Initialize map components
	let viewMap = 'main-map';

	// Set up map and interactions on component mount
	onMount(async () => {
		// Create and configure map layers
		const osmLayer = new TileLayer({ source: new OSM() });

		const vector = new VectorLayer({
			source: $vectorPolySource
		});

		const startPointLayer = new VectorLayer({
			source: $startPointSource,
			style: new Style({
				image: new Circle({
					radius: 7,
					fill: new Fill({
						color: 'rgba(255, 0, 0, 0.7)'
					})
				})
			})
		});

		$discretizedAreaLayer = new VectorLayer({
			source: new VectorSource(),
			style: new Style({
				image: new Circle({
					radius: 5,
					fill: new Fill({
						color: 'rgba(0, 255, 0, 0.7)'
					})
				})
			})
		});

		$planLayer = new VectorLayer({
			source: new VectorSource(),
			style: new Style({
				stroke: new Stroke({
					color: 'rgba(0, 0, 255, 0.7)',
					width: 3
				})
			})
		});

		// Initialize map with target element, layers, and view settings
		map.set(
			new Map({
				target: viewMap,
				layers: [osmLayer, vector, startPointLayer, $discretizedAreaLayer, $planLayer],
				view: new View({
					center: [0, 0],
					zoom: 2
				})
			})
		);

		// Initialize and add interactions to the map
		drawInteraction.set(
			new Draw({
				source: $vectorPolySource,
				type: 'Polygon'
			})
		);

		modifyInteraction.set(new Modify({ source: $vectorPolySource }));

		snapInteraction.set(new Snap({ source: $vectorPolySource }));
	});
</script>

<div id={viewMap} class="map" />

<style>
	@import '../styles/map-style.css';
	@import '../global.css';
	.map {
		margin: 0;
		padding: 0;
		width: 100%;
		height: 100%;
	}
</style>
