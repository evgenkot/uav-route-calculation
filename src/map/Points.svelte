<script lang="ts">
	import Point from 'ol/geom/Point';
	import Feature from 'ol/Feature';
	// Transform imports
	import { transform } from 'ol/proj';
	import proj4 from 'proj4';
	import { register } from 'ol/proj/proj4';

	import {
		utmZone,
		map,
		drawInteraction,
		modifyInteraction,
		snapInteraction,
		startPointSource,
		startSelected,
		isDrawing
	} from './store';

	// Set the UTM zone based on the starting point coordinates
	function setZone(coordinates: number[]) {
		$startSelected = true;
		const wgs84Coordinates = transform(coordinates, 'EPSG:3857', 'EPSG:4326');
		const lat = wgs84Coordinates[1];
		const lon = wgs84Coordinates[0];
		const utmZoneNumber = Math.floor((lon + 180) / 6) + 1;
		const isNorthernHemisphere = lat >= 0;

		let zone = `EPSG:326${isNorthernHemisphere ? '' : '1'}${String(utmZoneNumber).padStart(
			2,
			'0'
		)}`;
		proj4.defs(
			zone,
			`+proj=utm +zone=${utmZoneNumber} ${
				isNorthernHemisphere ? '+north' : '+south'
			} +ellps=WGS84 +datum=WGS84 +units=m +no_defs`
		);
		// Also update store var
		utmZone.set(zone);
		register(proj4);
	}

	function setStartingPoint() {
		// Remove other interactions
		$map.removeInteraction($drawInteraction);
		$map.removeInteraction($modifyInteraction);
		$map.removeInteraction($snapInteraction);

		// Add a click event listener to the map
		$map.once('click', (event) => {
			const coordinates = event.coordinate;
			const startPoint = new Feature({
				geometry: new Point(coordinates)
			});

			// Remove existing starting point features from the source
			$startPointSource.clear();

			// Add the new starting point feature to the source
			$startPointSource.addFeature(startPoint);
			setZone(coordinates);
		});
	}
</script>

<button
	on:click={setStartingPoint}
	disabled={!!$isDrawing}
	class="{$startSelected ? 'done' : 'todo'} rmenu-category">Set start</button
><br />

<style>
	.done {
		background: hsl(115, 100%, 68%);
	}

	.todo {
		background: hwb(14 30% 0%);
	}
</style>
