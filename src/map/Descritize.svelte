<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import {
		altitudeValue,
		overlapValue,
		selectedCamera,
		selectedUav,
		startPointSource,
		utmZone,
		vectorPolySource,
		discretizedAreaLayer,
		areaDiscretized,
		areaSelected,
		startSelected,
		altitudeSelected,
		discretizedArea,
		startingPoint,
		discretizationDirection,
		Algorithm,
		selectedAlgorithm
	} from './store';
	import { transform } from 'ol/proj';
	import { Point, type Polygon } from 'ol/geom';
	import type { Coordinate } from 'ol/coordinate';
	import { Feature } from 'ol';

	let visible = true;
	function toggleVisible() {
		visible = !visible;
	}

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

		const utmCoordinates: number[][][] = [];

		features.forEach((feature) => {
			const polygon = feature.getGeometry() as Polygon;
			const coordinates = polygon.getCoordinates();

			const utmPolygon = coordinates.map((ring: Coordinate[]) =>
				ring.map((coord) => {
					const wgs84Coord = transform(coord, 'EPSG:3857', 'EPSG:4326');
					const utmCoord = transform(wgs84Coord, 'EPSG:4326', $utmZone);
					return [utmCoord[0], utmCoord[1]]; // Ensure the coordinates are numbers
				})
			);

			utmCoordinates.push(utmPolygon[0]); // Push the entire polygon into utmCoordinates
		});

		return utmCoordinates;
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

	async function discretize() {
		const vertices = getVertices();
		startingPoint.set(getStartingPointCoordinates());

		if (vertices === null) {
			alert('getVertices function is not available. Please make sure the map is loaded.');
			return;
		}

		if (vertices.length <= 0) {
			alert('Area not set');
			return;
		}

		if ($startingPoint === null) {
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

		try {
			const result = await invoke('discretize_area', {
				polygons: vertices,
				photoWidth: photoWidth,
				photoHeight: photoHeight,
				directionDegrees: $discretizationDirection,
				checkInside: $selectedAlgorithm != Algorithm.RectangularAreas
			});
			$discretizedArea = result as number[][][][];
			console.log(discretizedArea);
		} catch (error) {
			alert('Error calling discretize_area. ' + error);
			return;
		}

		updateDiscretizedLayer(
			$discretizedArea.flatMap((innerArr) => innerArr).flatMap((innerArr) => innerArr)
		);
		areaDiscretized.set(true);
	}
</script>

<button on:click={toggleVisible} class="{areaDiscretized ? 'done' : 'todo'} rmenu-category"
	>{visible ? 'Area discretization' : 'Show area discretization'}</button
><br />
{#if visible}
	<div class="input-row">
		<label for="direction-field">Discretization Direction:</label>
		<input
			type="number"
			id="direction-field"
			min="0"
			max="360"
			step="0.1"
			bind:value={$discretizationDirection}
		/>
	</div>
	<button on:click={discretize} disabled={!($areaSelected && $startSelected && $altitudeSelected)}
		>Discretize</button
	><br />
{/if}

<style>
	.done {
		background: hsl(115, 100%, 68%);
	}

	.todo {
		background: hwb(14 30% 0%);
	}
</style>
