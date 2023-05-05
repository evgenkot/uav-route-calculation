<script lang="ts">
	//OL imports
	import Map from 'ol/Map';
	import View from 'ol/View';
	import { OSM, Vector as VectorSource, XYZ } from 'ol/source';
	import { Tile as TileLayer, Vector as VectorLayer } from 'ol/layer';
	import Draw from 'ol/interaction/Draw';
	import Modify from 'ol/interaction/Modify';
	import Snap from 'ol/interaction/Snap';
	import { Polygon } from 'ol/geom';
	import { onMount } from 'svelte';
	import Point from 'ol/geom/Point';
	import Style from 'ol/style/Style';
	import Feature from 'ol/Feature';
	import Circle from 'ol/style/Circle';
	import Fill from 'ol/style/Fill';
	import LineString from 'ol/geom/LineString';
	import Stroke from 'ol/style/Stroke';

	// Transform imports
	import { transform } from 'ol/proj';
	import proj4 from 'proj4';
	import { register } from 'ol/proj/proj4';

	// Tauri imports
	import { invoke } from '@tauri-apps/api/tauri';

	let viewMap = 'main-map';
	let map: Map;
	let drawInteraction: Draw;
	let modifyInteraction: Modify;
	let snapInteraction: Snap;
	let startPointSource = new VectorSource();
	let discretizedAreaLayer = new VectorLayer();
	let nearestNeighborLayer = new VectorLayer();

	onMount(async () => {
		const osmLayer = new TileLayer({ source: new OSM() });

		const source = new VectorSource({ wrapX: false });

		const vector = new VectorLayer({
			source: source
		});

		const startPointLayer = new VectorLayer({
			source: startPointSource,
			style: new Style({
				image: new Circle({
					radius: 7,
					fill: new Fill({
						color: 'rgba(255, 0, 0, 0.7)'
					})
				})
			})
		});

		discretizedAreaLayer = new VectorLayer({
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

		nearestNeighborLayer = new VectorLayer({
			source: new VectorSource(),
			style: new Style({
				stroke: new Stroke({
					color: 'rgba(0, 0, 255, 0.7)',
					width: 3
				})
			})
		});

		map = new Map({
			target: viewMap,
			layers: [osmLayer, vector, startPointLayer, discretizedAreaLayer, nearestNeighborLayer],
			view: new View({
				center: [0, 0],
				zoom: 2
			})
		});

		drawInteraction = new Draw({
			source: source,
			type: 'Polygon'
		});

		modifyInteraction = new Modify({ source: source });
		map.addInteraction(modifyInteraction);

		snapInteraction = new Snap({ source: source });
		map.addInteraction(snapInteraction);
	});

	function undo() {
		drawInteraction.removeLastPoint();
	}

	function enableNavigation() {
		map.removeInteraction(drawInteraction);
		map.removeInteraction(modifyInteraction);
		map.removeInteraction(snapInteraction);
	}

	function enableDrawing() {
		map.addInteraction(drawInteraction);
		map.addInteraction(modifyInteraction);
		map.addInteraction(snapInteraction);
	}

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

	
	function enableStartingPoint() {
		// Remove other interactions
		map.removeInteraction(drawInteraction);
		map.removeInteraction(modifyInteraction);
		map.removeInteraction(snapInteraction);

		// Add a click event listener to the map
		map.once('click', (event) => {
			const coordinates = event.coordinate;
			const startPoint = new Feature({
				geometry: new Point(coordinates)
			});

			// Remove existing starting point features from the source
			startPointSource.clear();

			// Add the new starting point feature to the source
			startPointSource.addFeature(startPoint);
		});
	}

	function getStartingPointCoordinates(): number[] | null {
		const features = startPointSource.getFeatures();
		if (features.length === 0) {
			return null;
		}

		const startingPointFeature = features[0];
		const geometry = startingPointFeature.getGeometry();

		if (geometry instanceof Point) {
			const startingPointCoordinates = geometry.getCoordinates();

			// Transform starting point coordinates to EPSG:4326 (longitude, latitude)
			const [longitude, latitude] = transform(startingPointCoordinates, 'EPSG:3857', 'EPSG:4326');

			// Define the UTM projection for the starting point
			defineUTMProjection(latitude, longitude);

			// Transform coordinates to the UTM zone projection
			const utmEpsgCode = getUTMEPSGCode(latitude, longitude);
			const startingPointCoordinatesInMeters = transform(
				startingPointCoordinates,
				'EPSG:3857',
				utmEpsgCode
			);
			// console.log(startingPointCoordinatesInMeters);
			return startingPointCoordinatesInMeters;
		}

		return null;
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
		// console.log(vertices);
		// invoke('receive_polygon_coordinates', { vertices });
		return vertices;
		// Send the coordinates to the Rust backend
	}

	function updateResultsLayer(discretizedArea: number[][], planResult: number[][]) {
		const discretizedAreaSource = discretizedAreaLayer.getSource();
		const nearestNeighborSource = nearestNeighborLayer.getSource();

		if (discretizedAreaSource == null) {
			alert('discretizedAreaSource not loaded');
			return;
		}
		if (nearestNeighborSource == null) {
			alert('nearestNeighborSource not loaded');
			return;
		}

		discretizedAreaSource.clear();
		nearestNeighborSource.clear();

		discretizedArea.forEach((point) => {
			const [longitude, latitude] = point;
			const utmEpsgCode = getUTMEPSGCode(latitude, longitude);

			// Define the UTM projection for the point
			defineUTMProjection(latitude, longitude);

			const coordinates = transform(point, utmEpsgCode, 'EPSG:3857');
			const feature = new Feature(new Point(coordinates));
			discretizedAreaSource.addFeature(feature);
		});

		const nearestNeighborLineCoordinates = planResult.map((point) => {
			const [longitude, latitude] = point;
			const utmEpsgCode = getUTMEPSGCode(latitude, longitude);

			// Define the UTM projection for the point
			defineUTMProjection(latitude, longitude);

			return transform(point, utmEpsgCode, 'EPSG:3857');
		});

		const nearestNeighborLine = new LineString(nearestNeighborLineCoordinates);
		const nearestNeighborLineFeature = new Feature(nearestNeighborLine);
		nearestNeighborSource.addFeature(nearestNeighborLineFeature);
	}

	async function calculate() {
		const vertices = getVertices();
		const startingPoint = getStartingPointCoordinates();

		if (vertices === undefined) {
			alert('getVertices function is not available. Please make sure the map is loaded.');
			return;
		}

		if (vertices.length <= 0) {
			alert('Area not set');
			return;
		}

		if (startingPoint === null) {
			alert('Starting point not set');
			return;
		}

		// console.log(vertices, startingPoint);
		const photoWidth = 40;
		const photoHeight = 30;
		let discretizedArea;
		let planResult;

		try {
			const result = await invoke('discretize_area', {
				polygon: vertices[0],
				photoWidth: photoWidth,
				photoHeight: photoHeight
			});
			discretizedArea = result as number[][];
			console.log(discretizedArea);
		} catch (error) {
			alert('Error calling discretize_area');
			return;
		}

		try {
			const result = await invoke('nearest_neighbor', {
				points: discretizedArea,
				startPoint: startingPoint
			});

			planResult = result as number[][];
			console.log(planResult);
		} catch (error) {
			alert('Error calling nearest Neighbor');
			return;
		}

		updateResultsLayer(discretizedArea, planResult);
	}
</script>

<div id={viewMap} class="map" />
<div class="toolbar">
	<button on:click={undo}>Undo</button>
	<button on:click={enableDrawing}>Draw</button>
	<button on:click={enableStartingPoint}>Set Starting Point</button>
	<button on:click={enableNavigation}>Navigation</button>
	<button on:click={calculate}>Calculate</button>
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
