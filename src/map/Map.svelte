<script lang="ts">
	//OL imports
	import Map from 'ol/Map';
	import View from 'ol/View';
	import { OSM, Vector as VectorSource } from 'ol/source';
	import { Tile as TileLayer, Vector as VectorLayer } from 'ol/layer';
	import Draw from 'ol/interaction/Draw';
	import Modify from 'ol/interaction/Modify';
	import Snap from 'ol/interaction/Snap';
	import type { Polygon } from 'ol/geom';
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

	// Store import
	import {
		altitudeValue,
		selectedCamera,
		selectedUav,
		overlapValue,
		selectedAlgorithm
	} from './store';
	import { Algorithm } from './store';

	let viewMap = 'main-map';
	let map: Map;
	let drawInteraction: Draw;
	let modifyInteraction: Modify;
	let snapInteraction: Snap;
	let startPointSource = new VectorSource();
	let vectorPolySource = new VectorSource({ wrapX: false });
	let discretizedAreaLayer = new VectorLayer();
	let planLayer = new VectorLayer();

	onMount(async () => {
		const osmLayer = new TileLayer({ source: new OSM() });

		// const source = new VectorSource({ wrapX: false });

		const vector = new VectorLayer({
			source: vectorPolySource
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

		planLayer = new VectorLayer({
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
			layers: [osmLayer, vector, startPointLayer, discretizedAreaLayer, planLayer],
			view: new View({
				center: [0, 0],
				zoom: 2
			})
		});

		drawInteraction = new Draw({
			source: vectorPolySource,
			type: 'Polygon'
		});

		modifyInteraction = new Modify({ source: vectorPolySource });
		map.addInteraction(modifyInteraction);

		snapInteraction = new Snap({ source: vectorPolySource });
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

	import { get as getProjection } from 'ol/proj';

	let zone = 'EPSG:3857';
	proj4.defs(zone, `+proj=utm +zone=1 +ellps=WGS84 +datum=WGS84 +units=m +no_defs`);
	register(proj4);

	function getStartingPointCoordinates(): number[] | null {
		const features = startPointSource.getFeatures();
		if (features.length === 0) {
			return null;
		}
		const point = features[0].getGeometry() as Point;
		const coordinates = point.getCoordinates();
		const wgs84Coordinates = transform(coordinates, 'EPSG:3857', 'EPSG:4326');
		const utmCoordinates = transform(wgs84Coordinates, 'EPSG:4326', zone);
		return utmCoordinates;
	}

	function getVertices(): number[][][] | null {
		const features = vectorPolySource.getFeatures();
		if (features.length === 0) {
			return null;
		}
		const polygon = features[0].getGeometry() as Polygon;
		const coordinates = polygon.getCoordinates();
		const utmCoordinates = coordinates.map((ring) =>
			ring.map((coord) => {
				const wgs84Coord = transform(coord, 'EPSG:3857', 'EPSG:4326');
				return transform(wgs84Coord, 'EPSG:4326', zone);
			})
		);
		return utmCoordinates;
	}
	function updateResultsLayer(discretizedArea: number[][], planResult: number[][]) {
		const discretizedAreaSource = discretizedAreaLayer.getSource();
		const planSource = planLayer.getSource();

		if (discretizedAreaSource === null || planSource === null) {
			alert('Layer sources not found');
			return;
		}

		discretizedAreaSource.clear();
		planSource.clear();

		const discretizedAreaFeatures = discretizedArea.map((coord) => {
			const wgs84Coord = transform(coord, zone, 'EPSG:4326');
			const webMercatorCoord = transform(wgs84Coord, 'EPSG:4326', 'EPSG:3857');
			return new Feature(new Point(webMercatorCoord));
		});

		const planResultLine = new LineString(
			planResult.map((coord) => {
				const wgs84Coord = transform(coord, zone, 'EPSG:4326');
				const webMercatorCoord = transform(wgs84Coord, 'EPSG:4326', 'EPSG:3857');
				return webMercatorCoord;
			})
		);

		const planResultLineFeature = new Feature(planResultLine);
		discretizedAreaSource.addFeatures(discretizedAreaFeatures);
		planSource.addFeature(planResultLineFeature);
	}

	function updatePlanLayer(planResult: number[][]) {
		const planSource = planLayer.getSource();

		if (planSource === null) {
			alert('Plan Layer sources not found');
			return;
		}

		planSource.clear();

		const planResultLine = new LineString(
			planResult.map((coord) => {
				const wgs84Coord = transform(coord, zone, 'EPSG:4326');
				const webMercatorCoord = transform(wgs84Coord, 'EPSG:4326', 'EPSG:3857');
				return webMercatorCoord;
			})
		);

		const planResultLineFeature = new Feature(planResultLine);
		planSource.addFeature(planResultLineFeature);
	}

	function updateDiscretizedLayer(discretizedArea: number[][]) {
		const discretizedAreaSource = discretizedAreaLayer.getSource();

		if (discretizedAreaSource === null) {
			alert('DiscretizedArea Source not found');
			return;
		}

		discretizedAreaSource.clear();

		const discretizedAreaFeatures = discretizedArea.map((coord) => {
			const wgs84Coord = transform(coord, zone, 'EPSG:4326');
			const webMercatorCoord = transform(wgs84Coord, 'EPSG:4326', 'EPSG:3857');
			return new Feature(new Point(webMercatorCoord));
		});

		discretizedAreaSource.addFeatures(discretizedAreaFeatures);
	}

	function setZone(coordinates: number[]) {
		const wgs84Coordinates = transform(coordinates, 'EPSG:3857', 'EPSG:4326');
		const lat = wgs84Coordinates[1];
		const lon = wgs84Coordinates[0];
		const utmZoneNumber = Math.floor((lon + 180) / 6) + 1;
		const isNorthernHemisphere = lat >= 0;

		zone = `EPSG:326${isNorthernHemisphere ? '' : '1'}${String(utmZoneNumber).padStart(2, '0')}`;
		proj4.defs(
			zone,
			`+proj=utm +zone=${utmZoneNumber} ${
				isNorthernHemisphere ? '+north' : '+south'
			} +ellps=WGS84 +datum=WGS84 +units=m +no_defs`
		);
		register(proj4);
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
			setZone(coordinates);
		});
	}

	async function calculate() {
		const vertices = getVertices();
		const startingPoint = getStartingPointCoordinates();

		if (vertices === null) {
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

		if (!$selectedCamera) {
			alert('Camera not set');
			return;
		}

		if (!$selectedUav) {
			alert('Uav not set');
			return;
		}

		if ($selectedUav.min_altitude > $altitudeValue || $selectedUav.max_altitude < $altitudeValue) {
			alert('Altitude out of range');
			return;
		}

		if ($selectedUav.min_altitude > $altitudeValue || $selectedUav.max_altitude < $altitudeValue) {
			alert('Altitude out of range');
			return;
		}
		const alt = $altitudeValue;

		const overlap = $overlapValue / 100;
		const tg = (angle: number) => Math.tan(angle * (Math.PI / 180));

		altitudeValue;
		const photoWidth = tg($selectedCamera.fov_x * 0.5) * 2 * alt * (1 - overlap);
		const photoHeight = (photoWidth * $selectedCamera.resolution_y) / $selectedCamera.resolution_x;
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

		updateDiscretizedLayer(discretizedArea);

		async function confirmBruteForce(): Promise<boolean> {
			return new Promise((resolve) => {
				const shouldExecute = window.confirm(
					'You are going to use Brute Force to calculate route, it will take a long time.'
				);
				resolve(shouldExecute);
			});
		}

		try {
			let result;
			switch ($selectedAlgorithm) {
				case Algorithm.NearestNeighbor: {
					result = await invoke('nearest_neighbor', {
						points: discretizedArea,
						startPoint: startingPoint
					});
					break;
				}
				case Algorithm.BruteForce: {
					const shouldExecute = await confirmBruteForce();
					if (shouldExecute) {
						result = await invoke('brute_force', {
							points: discretizedArea,
							startPoint: startingPoint
						});
					} else {
						return;
					}
					break;
				}
				default:
					alert('No algo selected');
					break;
			}

			planResult = result as number[][];
			console.log(planResult);
		} catch (error) {
			alert('Error calling calculation');
			return;
		}

		console.log('discretizedArea', discretizedArea);
		console.log('startingPoint', startingPoint);
		updatePlanLayer(planResult);
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
