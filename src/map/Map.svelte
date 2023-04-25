<script lang="ts">
	import Map from 'ol/Map';
	import View from 'ol/View';
	import { OSM, Vector as VectorSource, XYZ } from 'ol/source';
	import { Tile as TileLayer, Vector as VectorLayer } from 'ol/layer';
	import Draw from 'ol/interaction/Draw';
	import Modify from 'ol/interaction/Modify';
	import Snap from 'ol/interaction/Snap';
	import { Polygon } from 'ol/geom';
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

	// Add the following imports
	import { transform } from 'ol/proj';
	import proj4 from 'proj4';
	import { register } from 'ol/proj/proj4';

	// Add the helper functions
	function getUTMZone(latitude: number, longitude: number): { zone: number; hemisphere: string } {
		const zone = (Math.floor((longitude + 180) / 6) % 60) + 1;
		const hemisphere = latitude >= 0 ? 'N' : 'S';
		return { zone, hemisphere };
	}

	function getUTMEPSGCode(latitude: number, longitude: number): string {
		const { zone, hemisphere } = getUTMZone(latitude, longitude);
		const epsgCode = `EPSG:326${zone < 10 ? '0' + zone : zone}${hemisphere}`;
		return epsgCode;
	}

	function defineUTMProjection(latitude: number, longitude: number): void {
		const { zone, hemisphere } = getUTMZone(latitude, longitude);
		const epsgCode = getUTMEPSGCode(latitude, longitude);
		if (!proj4.defs(epsgCode)) {
			proj4.defs(
				epsgCode,
				`+proj=utm +zone=${zone} +${hemisphere} +ellps=WGS84 +datum=WGS84 +units=m +no_defs`
			);
			register(proj4);
		}
	}

	function getVertices(): number[][][] | undefined {
		const vectorLayer = map
			.getLayers()
			.getArray()
			.find((layer) => layer instanceof VectorLayer) as VectorLayer<VectorSource>;
		const source = vectorLayer.getSource() as VectorSource;
		const features = source.getFeatures();
		const vertices = features
			.filter((feature) => feature.getGeometry() instanceof Polygon)
			.map((feature) => {
				const geometry = feature.getGeometry();
				if (geometry instanceof Polygon) {
					// Get the coordinates in EPSG:3857
					const coordinates3857 = geometry.getCoordinates()[0];

					// Get the first coordinate of the polygon
					const [firstLongitude, firstLatitude] = transform(
						coordinates3857[0],
						'EPSG:3857',
						'EPSG:4326'
					);

					// Define the UTM projection for the area of interest
					defineUTMProjection(firstLatitude, firstLongitude);

					// Transform coordinates to the UTM zone projection
					const utmEpsgCode = getUTMEPSGCode(firstLatitude, firstLongitude);
					return coordinates3857.map((coordinate) =>
						transform(coordinate, 'EPSG:3857', utmEpsgCode)
					);
				}
				return undefined;
			})
			.filter((coord) => coord !== undefined) as number[][][];

		return vertices;
		// Send the coordinates to the Rust backend
		// invoke('receive_polygon_coordinates', { vertices });
	}


	
</script>

<div id={viewMap} class="map" />
<div class="toolbar">
	<button on:click={undo}>Undo</button>
	<button on:click={enableDrawing}>Draw</button>
	<button on:click={enableNavigation}>Navigation</button>
	<button on:click={getVertices}>Calculate</button>
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
