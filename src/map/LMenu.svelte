<script lang="ts">
	import { onMount } from 'svelte';

	import { writable } from 'svelte/store';
	import { invoke } from '@tauri-apps/api/tauri';
	// import { Vec } from '@tauri-apps/api/rust';

	interface Uav {
		id: number;
		name: string;
		max_payload_mass: number;
		flight_duration: number;
		takeoff_speed: number;
		flight_speed: number;
		min_altitude: number;
		max_altitude: number;
	}

	// async function fetchUavs() {
	// 	try {
	// 		const result = await invoke<Uav[]>('get_uavs_vec');
	// 		// Handle the result, e.g., update the UI or populate a variable with the data
	// 		console.log(result);
	// 	} catch (error) {
	// 		console.error('Failed to fetch UAVs:', error);
	// 	}
	// }
	let uavs: Uav[] = [];
	let selectedUav: Uav | null = null;

	async function fetchUavs() {
		try {
			const result = await invoke<Uav[]>('get_uavs_vec');
			uavs = result;
			selectedUav = uavs.length > 0 ? uavs[0] : null;
			console.log(uavs);
		} catch (error) {
			console.error('Failed to fetch UAVs:', error);
		}
	}

	let uavOnEdit = false;

	function onUavFieldChange() {
		uavOnEdit = true;
	}
	function isUavValid(uav: Uav): boolean {
		// Check if the UAV name is not empty
		if (!uav.name || uav.name.trim() === '') {
			alert('UAV name cannot be empty');
			return false;
		}

		// Check if the maximum payload mass is within a reasonable range (e.g., 100g to 10kg)
		if (uav.max_payload_mass < 100 || uav.max_payload_mass > 10000) {
			alert('Max payload mass should be between 100g and 10kg');
			return false;
		}

		// Check if the flight duration is within a reasonable range (e.g., 1 minute to 8 hours)
		if (uav.flight_duration < 60 || uav.flight_duration > 28800) {
			alert('Flight duration should be between 1 minute and 8 hours');
			return false;
		}

		// Check if the takeoff and flight speeds are within reasonable ranges (e.g., 0.5m/s to 50m/s)
		if (
			uav.takeoff_speed < 0.5 ||
			uav.takeoff_speed > 50 ||
			uav.flight_speed < 0.5 ||
			uav.flight_speed > 50
		) {
			alert('Takeoff and flight speeds should be between 0.5m/s and 50m/s');
			return false;
		}

		// Check if the minimum and maximum altitudes are within reasonable ranges (e.g., 2m to 5000m)
		if (
			uav.min_altitude < 2 ||
			uav.min_altitude > 5000 ||
			uav.max_altitude < 5 ||
			uav.max_altitude > 5000
		) {
			alert('Minimum and maximum altitudes should be between 2m and 5000m');
			return false;
		}

		// Check if the minimum altitude is less than or equal to the maximum altitude
		if (uav.min_altitude >= uav.max_altitude) {
			alert('Minimum altitude should be less than the maximum altitude');
			return false;
		}

		return true;
	}

	async function saveUav() {
		const uav: Uav = {
			id: parseInt((document.getElementById('uav_id') as HTMLInputElement).value),
			name: (document.getElementById('uav_name') as HTMLInputElement).value,
			max_payload_mass: parseInt(
				(document.getElementById('uav_max_payload_mass') as HTMLInputElement).value
			),
			flight_duration: parseInt(
				(document.getElementById('uav_flight_duration') as HTMLInputElement).value
			),
			takeoff_speed: parseFloat(
				(document.getElementById('uav_takeoff_speed') as HTMLInputElement).value
			),
			flight_speed: parseFloat(
				(document.getElementById('uav_flight_speed') as HTMLInputElement).value
			),
			min_altitude: parseFloat(
				(document.getElementById('uav_min_altitude') as HTMLInputElement).value
			),
			max_altitude: parseFloat(
				(document.getElementById('uav_max_altitude') as HTMLInputElement).value
			)
		};

		if (isUavValid(uav)) {
			// await invoke('update_uav', uav);
			const response = await invoke<string>('update_uav', { uav }); // Update the payload here
			// const response = await invoke<string>('update_uav', { payload: uav });
			console.log('norm');
			console.log(uav)

			uavOnEdit = false;
		} else {
			console.error('Invalid UAV data');
			// alert('UAV validation failed. Please check the input values and try again.');
		}
	}

	let isActive = false;
	let sidenavWidth = 0;

	function toggleMenu() {
		isActive = !isActive;
		if (isActive) {
			sidenavWidth = 270;
		} else {
			sidenavWidth = 0;
		}
	}

	function toggleUAVBlock() {
		let block = document.getElementById('uav');
		if (block !== null) {
			if (block.style.display === 'none') {
				block.style.display = 'block';
			} else {
				block.style.display = 'none';
			}
		}
	}

	let isEditModeUAV = false;
	function toggleEditModeUAV() {
		isEditModeUAV = !isEditModeUAV;
	}

	function toggleCameraBlock() {
		let block = document.getElementById('camera');
		if (block !== null) {
			if (block.style.display === 'none') {
				block.style.display = 'block';
			} else {
				block.style.display = 'none';
			}
		}
	}

	function toggleAltitudeBlock() {
		let block = document.getElementById('altitude');
		if (block !== null) {
			if (block.style.display === 'none') {
				block.style.display = 'block';
			} else {
				block.style.display = 'none';
			}
		}
	}

	onMount(() => {
		fetchUavs();
	});
</script>

<div class="sidenav" style={`width: ${sidenavWidth}px`}>
	<h1>Menu</h1>
	<select bind:value={selectedUav} on:change={() => {}}>
		{#each uavs as uav (uav.id)}
			<option value={uav}>{uav.name}</option>
		{/each}
	</select>
	<button on:click={toggleUAVBlock} class="toggle-display">UAV detatils</button>
	<input type="checkbox" on:change={toggleEditModeUAV} class="edit-mode-checkbox" />
	<label for="edit-mode-uav" class="edit-mode-label">Edit Mode</label>
	<div class="block" id="uav">
		<div class="parameters">
			<label for="uav_id" class="label">ID:</label>
			<input
				type="number"
				class="input"
				id="uav_id"
				value={selectedUav ? selectedUav.id : ''}
				readonly
			/>
		</div>
		<div class="parameters">
			<label for="uav_name" class="label">Name:</label>
			<input
				type="text"
				class="input"
				id="uav_name"
				value={selectedUav ? selectedUav.name : ''}
				readonly={!isEditModeUAV}
				on:input={onUavFieldChange}
			/>
		</div>
		<div class="parameters">
			<label for="uav_max_payload_mass" class="label">Max Payload Mass:</label>
			<input
				type="number"
				class="input"
				id="uav_max_payload_mass"
				value={selectedUav ? selectedUav.max_payload_mass : ''}
				readonly={!isEditModeUAV}
				on:input={onUavFieldChange}
			/>
		</div>
		<div class="parameters">
			<label for="uav_flight_duration" class="label">Flight Duration:</label>
			<input
				type="number"
				class="input"
				id="uav_flight_duration"
				value={selectedUav ? selectedUav.flight_duration : ''}
				readonly={!isEditModeUAV}
				on:input={onUavFieldChange}
			/>
		</div>
		<div class="parameters">
			<label for="takeoff_speed" class="label">Takeoff speed:</label>
			<input
				type="number"
				class="input"
				id="uav_takeoff_speed"
				value={selectedUav ? selectedUav.takeoff_speed : ''}
				readonly={!isEditModeUAV}
				on:input={onUavFieldChange}
			/>
		</div>
		<div class="parameters">
			<label for="flight_speed" class="label">Flight speed:</label>
			<input
				type="number"
				class="input"
				id="uav_flight_speed"
				value={selectedUav ? selectedUav.flight_speed : ''}
				readonly={!isEditModeUAV}
				on:input={onUavFieldChange}
			/>
		</div>
		<div class="parameters">
			<label for="min_altitude" class="label">Min altitude:</label>
			<input
				type="number"
				class="input"
				id="uav_min_altitude"
				value={selectedUav ? selectedUav.min_altitude : ''}
				readonly={!isEditModeUAV}
				on:input={onUavFieldChange}
			/>
		</div>
		<div class="parameters">
			<label for="max_altitude" class="label">Max altitude:</label>
			<input
				type="number"
				class="input"
				id="uav_max_altitude"
				value={selectedUav ? selectedUav.max_altitude : ''}
				readonly={!isEditModeUAV}
				on:input={onUavFieldChange}
			/>
		</div>
		<button class="save-uav" on:click={saveUav} disabled={!uavOnEdit}>Save</button>
	</div>

	<button on:click={toggleCameraBlock} class="toggle-display">Camera detatils</button>
	<div class="block" id="camera">
		<div class="parameters">
			<label for="camera_id" class="label">ID:</label>
			<input type="text" class="input" id="camera_id" />
		</div>
		<div class="parameters">
			<label for="camera_name" class="label">Name:</label>
			<input type="text" class="input" id="camera_name" />
		</div>
		<div class="parameters">
			<label for="camera_mass" class="label">Mass:</label>
			<input type="text" class="input" id="camera_mass" />
		</div>
		<div class="parameters">
			<label for="camere_fov_x" class="label">Fov X:</label>
			<input type="text" class="input" id="camere_fov_x" />
		</div>
		<div class="parameters">
			<label for="camere_fov_y" class="label">Fov Y:</label>
			<input type="text" class="input" id="camere_fov_y" />
		</div>
		<div class="parameters">
			<label for="camera_resolution_x" class="label">Resolution X:</label>
			<input type="text" class="input" id="camera_resolution_x" />
		</div>
		<div class="parameters">
			<label for="camera_resolution_y" class="label">Resolution Y:</label>
			<input type="text" class="input" id="camera_resolution_y" />
		</div>
	</div>

	<button on:click={toggleAltitudeBlock} class="toggle-display">Altitude detatils</button>
	<div class="block" id="altitude">
		<div class="parameters">
			<label for="flight_altitude " class="label">Flight altitude:</label>
			<input type="text" class="input" id="altitude_flight_altitude " />
		</div>
		<div class="parameters">
			<label for="ground_level" class="label">Ground level:</label>
			<input type="text" class="input" id="altitude_ground_level" />
		</div>
		<div class="parameters">
			<label for="altitude_sea_level" class="label">Sea level:</label>
			<input type="text" class="input" id="altitude_sea_level" />
		</div>
	</div>
</div>

<div class="toggle-container">
	<button on:click={toggleMenu} class="toggle-button">
		Details {isActive ? 'Hide' : 'Show'}
	</button>
</div>

<style>
	@import '../styles/side-menu.css';
	.sidenav {
		left: 0;
	}
	.toggle-container {
		left: 0;
	}
</style>
