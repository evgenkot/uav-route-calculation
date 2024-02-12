<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import {
		Algorithm,
		altitudeValue,
		missionDuration,
		photoCount,
		planInMeters,
		planLayer,
		routeLength,
		selectedAlgorithm,
		selectedUav,
		utmZone,
		discretizedAreaLayer,
		discretizedArea,
		startingPoint,
		planResult,

		areaDiscretized,

		areaSelected,
		startSelected,
		altitudeSelected,

	} from './store';
	import { transform } from 'ol/proj';
	import { LineString, Point, type Polygon } from 'ol/geom';
	import { Feature } from 'ol';

	async function calculate() {
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
						points: $discretizedArea.flatMap((innerArr) => innerArr),
						startPoint: $startingPoint
					});
					break;
				}
				case Algorithm.BruteForce: {
					const shouldExecute = await confirmBruteForce();
					if (shouldExecute) {
						result = await invoke('brute_force', {
							points: $discretizedArea.flatMap((innerArr) => innerArr),
							startPoint: $startingPoint
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

			$planResult = result as number[][];
			planInMeters.set($planResult);
			routeLength.set(
				await invoke('calculate_distance', {
					points: $planResult
				})
			);
			console.log($planResult);
		} catch (error) {
			alert('Error calling calculation. ' + error);
			return;
		}

		photoCount.set($discretizedArea.flatMap((innerArr) => innerArr).length);

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

		console.log('discretizedArea', $discretizedArea);
		console.log('startingPoint', startingPoint);
		updatePlanLayer($planResult);
	}

	// Update the results layer with the discretized area and the calculated flight plan
	function updateResultsLayer($discretizedArea: number[][], $planResult: number[][]) {
		const discretizedAreaSource = $discretizedAreaLayer.getSource();
		const planSource = $planLayer.getSource();

		if (discretizedAreaSource === null || planSource === null) {
			alert('Layer sources not found');
			return;
		}

		discretizedAreaSource.clear();
		planSource.clear();

		const discretizedAreaFeatures = $discretizedArea.map((coord) => {
			const wgs84Coord = transform(coord, $utmZone, 'EPSG:4326');
			const webMercatorCoord = transform(wgs84Coord, 'EPSG:4326', 'EPSG:3857');
			return new Feature(new Point(webMercatorCoord));
		});

		const planResultLine = new LineString(
			$planResult.map((coord) => {
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
	function updatePlanLayer($planResult: number[][]) {
		const planSource = $planLayer.getSource();

		if (planSource === null) {
			alert('Plan Layer sources not found');
			return;
		}

		planSource.clear();

		const planResultLine = new LineString(
			$planResult.map((coord) => {
				const wgs84Coord = transform(coord, $utmZone, 'EPSG:4326');
				const webMercatorCoord = transform(wgs84Coord, 'EPSG:4326', 'EPSG:3857');
				return webMercatorCoord;
			})
		);

		const planResultLineFeature = new Feature(planResultLine);
		planSource.addFeature(planResultLineFeature);
	}
</script>

<button on:click={calculate} disabled={!($areaDiscretized && $areaSelected && $startSelected && $altitudeSelected)}>Calculate</button>

<style>
	.done {
		background: hsl(115, 100%, 68%);
	}

	.todo {
		background: hwb(14 30% 0%);
	}
</style>
