<script lang="ts">
	// OL imports
	import { transform } from 'ol/proj';

	// Store imports
	import { routeLength, missionDuration, photoCount, utmZone, planInMeters } from './store';

	// Tauri API
	import { save } from '@tauri-apps/api/dialog';
	import { writeFile } from '@tauri-apps/api/fs';

	let routeLengthValue: number;
	let missionDurationValue: number;
	let photoCountValue: number;

	routeLength.subscribe((value) => {
		routeLengthValue = value;
	});

	missionDuration.subscribe((value) => {
		missionDurationValue = value;
	});

	photoCount.subscribe((value) => {
		photoCountValue = value;
	});

	async function exportToGeoJSON() {
		// Convert planInMeters to WGS84
		let wgs84Coordinates: number[][] = [];
		$planInMeters.forEach((coord) => {
			const wgs84Coord = transform(coord, $utmZone, 'EPSG:4326');
			wgs84Coordinates.push(wgs84Coord);
		});

		const geojson = {
			type: 'Feature',
			properties: {},
			geometry: {
				type: 'LineString',
				coordinates: wgs84Coordinates
			}
		};

		// Save GeoJSON object to file
		try {
			const filePath = await save({
				filters: [
					{
						name: 'route',
						extensions: ['json']
					}
				]
			});

			if (filePath) {
				await writeFile({
					path: filePath,
					contents: JSON.stringify(geojson, null, 2)
				});
				console.log('Exported to GeoJSON:', filePath);
			} else {
				console.log('Export cancelled');
			}
		} catch (error) {
			console.error('Failed to save GeoJSON:', error);
			alert('An error occurred while saving the GeoJSON file. Please try again.');
		}
	}
</script>

<div>
	<h2>Mission Parameters</h2>
	<ul>
		<li>Route Length: {routeLengthValue} m.</li>
		<li>Mission Duration: {missionDurationValue} s.</li>
		<li>Number of Photos: {photoCountValue}</li>
	</ul>
	<button on:click={exportToGeoJSON}>Export to GeoJSON</button>
</div>
