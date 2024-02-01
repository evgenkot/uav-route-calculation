<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import {
		Algorithm,
		altitudeValue,
		missionDuration,
		overlapValue,
		photoCount,
		planInMeters,
		planLayer,
		routeLength,
		selectedAlgorithm,
		selectedCamera,
		selectedUav,
		startPointSource,
		utmZone,
		vectorPolySource,
		discretizedAreaLayer
	} from './store';
	import { transform } from 'ol/proj';
	import { LineString, Point, type Polygon } from 'ol/geom';
	import type { Coordinate } from 'ol/coordinate';
	import { Feature } from 'ol';

	// Function to get the starting point coordinates in UTM
	function getStartingPointCoordinates(): number[] | null {
		const features = $startPointSource.getFeatures();
		if (features.length === 0) {
			return null;
		}
		const point = features[0].getGeometry() as Point;
		const coordinates = point.getCoordinates();
		const wgs84Coordinates = transform(coordinates, 'EPSG:3857', 'EPSG:4326');
		const utmCoordinates = transform(wgs84Coordinates, 'EPSG:4326', $utmZone);
		return utmCoordinates;
	}

	// Function to get the vertices of the drawn polygon in UTM
	function getVertices(): number[][][] | null {
		const features = $vectorPolySource.getFeatures();
		if (features.length === 0) {
			return null;
		}
		const polygon = features[0].getGeometry() as Polygon;
		const coordinates = polygon.getCoordinates();
		const utmCoordinates = coordinates.map((ring: Coordinate[]) =>
			ring.map((coord) => {
				const wgs84Coord = transform(coord, 'EPSG:3857', 'EPSG:4326');
				return transform(wgs84Coord, 'EPSG:4326', $utmZone);
			})
		);
		return utmCoordinates;
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

		if ($selectedUav.max_payload_mass < $selectedCamera.mass) {
			alert('Uav max payload mass less then camera mass');
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
			planInMeters.set(planResult);
			routeLength.set(
				await invoke('calculate_distance', {
					points: planResult
				})
			);
			console.log(planResult);
		} catch (error) {
			alert('Error calling calculation');
			return;
		}

		photoCount.set(discretizedArea.length);

		if ($selectedUav) {
			missionDuration.set(
				$routeLength / $selectedUav.flight_speed + $altitudeValue / $selectedUav.takeoff_speed
			);
			if ($missionDuration > $selectedUav.flight_duration) {
				alert(
					'Mission duration exceeds what the drone is capable of flying, it is recommended to reduce the area.'
				);
			}
		}

		console.log('discretizedArea', discretizedArea);
		console.log('startingPoint', startingPoint);
		updatePlanLayer(planResult);
	}

	// Update the results layer with the discretized area and the calculated flight plan
	function updateResultsLayer(discretizedArea: number[][], planResult: number[][]) {
		const discretizedAreaSource = $discretizedAreaLayer.getSource();
		const planSource = $planLayer.getSource();

		if (discretizedAreaSource === null || planSource === null) {
			alert('Layer sources not found');
			return;
		}

		discretizedAreaSource.clear();
		planSource.clear();

		const discretizedAreaFeatures = discretizedArea.map((coord) => {
			const wgs84Coord = transform(coord, $utmZone, 'EPSG:4326');
			const webMercatorCoord = transform(wgs84Coord, 'EPSG:4326', 'EPSG:3857');
			return new Feature(new Point(webMercatorCoord));
		});

		const planResultLine = new LineString(
			planResult.map((coord) => {
				const wgs84Coord = transform(coord, $utmZone, 'EPSG:4326');
				const webMercatorCoord = transform(wgs84Coord, 'EPSG:4326', 'EPSG:3857');
				return webMercatorCoord;
			})
		);

		const planResultLineFeature = new Feature(planResultLine);
		discretizedAreaSource.addFeatures(discretizedAreaFeatures);
		planSource.addFeature(planResultLineFeature);
	}

	// Update the plan layer with the calculated flight plan
	function updatePlanLayer(planResult: number[][]) {
		const planSource = $planLayer.getSource();

		if (planSource === null) {
			alert('Plan Layer sources not found');
			return;
		}

		planSource.clear();

		const planResultLine = new LineString(
			planResult.map((coord) => {
				const wgs84Coord = transform(coord, $utmZone, 'EPSG:4326');
				const webMercatorCoord = transform(wgs84Coord, 'EPSG:4326', 'EPSG:3857');
				return webMercatorCoord;
			})
		);

		const planResultLineFeature = new Feature(planResultLine);
		planSource.addFeature(planResultLineFeature);
	}

	// Update the discretized area layer with the discretized area points
	function updateDiscretizedLayer(discretizedArea: number[][]) {
		const discretizedAreaSource = $discretizedAreaLayer.getSource();

		if (discretizedAreaSource === null) {
			alert('DiscretizedArea Source not found');
			return;
		}

		discretizedAreaSource.clear();

		const discretizedAreaFeatures = discretizedArea.map((coord) => {
			const wgs84Coord = transform(coord, $utmZone, 'EPSG:4326');
			const webMercatorCoord = transform(wgs84Coord, 'EPSG:4326', 'EPSG:3857');
			return new Feature(new Point(webMercatorCoord));
		});

		discretizedAreaSource.addFeatures(discretizedAreaFeatures);
	}
</script>

<button on:click={calculate}>Calculate</button>

<style>
	.done {
		background: hsl(115, 100%, 68%);
	}

	.todo {
		background: hwb(14 30% 0%);
	}
</style>
