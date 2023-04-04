<script lang="ts">
	import { onMount } from 'svelte';

	import { invoke } from '@tauri-apps/api/tauri';

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

	async function updateUav() {
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
			let response = await invoke<string>('update_uav', { uav }); // Update the payload here
			if (response != 'Ok') {
				alert(response);
			} else {
				// Find the index of the UAV in the local list with the same ID
				const index = uavs.findIndex((item) => item.id === uav.id);

				// Update the local UAV list
				if (index !== -1) {
					uavs[index] = uav;
					uavs = [...uavs]; // Trigger reactivity by creating a new array reference
				}

				selectedUav = uavs.length > 0 ? uavs[index] : null;
				uavOnEdit = false;
			}
		} else {
			console.error('Invalid UAV data');
		}
	}

	async function newUav() {
		const uav: Uav = {
			id: 0,
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
			let response = await invoke<string>('new_uav', { uav }); // Update the payload here
			if (response != 'Ok') {
				alert(response);
			} else {
				await fetchUavs();
				selectedUav = uavs.length > 0 ? uavs[uavs.length - 1] : null;
				uavOnEdit = false;
			}
		} else {
			console.error('Invalid UAV data');
		}
	}

	async function deleteUav() {
		const uav: Uav = {
			id: parseInt((document.getElementById('uav_id') as HTMLInputElement).value),
			name: 'uav_name',
			max_payload_mass: 0,
			flight_duration: 0,
			takeoff_speed: 0,
			flight_speed: 0,
			min_altitude: 0,
			max_altitude: 0
		};
		let response = await invoke<string>('delete_uav', { uav });
		if (response != 'Ok') {
			alert(response);
		} else {
			const index = uavs.findIndex((item) => item.id === uav.id);

			// Update the local UAV list

			fetchUavs();

			selectedUav = uavs.length > 0 ? uavs[0] : null;
			uavOnEdit = false;
		}
	}

	function undoUav() {
		if (uavOnEdit && selectedUav) {
			(document.getElementById('uav_name') as HTMLInputElement).value = selectedUav.name;
			(document.getElementById('uav_max_payload_mass') as HTMLInputElement).value =
				selectedUav.max_payload_mass.toString();
			(document.getElementById('uav_flight_duration') as HTMLInputElement).value =
				selectedUav.flight_duration.toString();
			(document.getElementById('uav_takeoff_speed') as HTMLInputElement).value =
				selectedUav.takeoff_speed.toString();
			(document.getElementById('uav_flight_speed') as HTMLInputElement).value =
				selectedUav.flight_speed.toString();
			(document.getElementById('uav_min_altitude') as HTMLInputElement).value =
				selectedUav.min_altitude.toString();
			(document.getElementById('uav_max_altitude') as HTMLInputElement).value =
				selectedUav.max_altitude.toString();
			uavOnEdit = false;
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

	onMount(() => {
		fetchUavs();
	});
</script>

<select bind:value={selectedUav} on:change={() => {}} disabled={uavOnEdit}>
	{#each uavs as uav (uav.id)}
		<option value={uav}>{uav.name}</option>
	{/each}
</select>
<button on:click={fetchUavs} class="fetch-uav">Fetch</button>
<button on:click={toggleUAVBlock} class="toggle-display">UAV detatils</button>
<div class="block" id="uav">
	<input
		type="checkbox"
		on:change={toggleEditModeUAV}
		class="edit-mode-checkbox"
		disabled={uavOnEdit}
	/>
	<label for="edit-mode-uav" class="edit-mode-label">Edit Mode</label>
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
	<button
		class="update-uav"
		on:click={updateUav}
		disabled={!uavOnEdit || uavs.length == 0 || !isEditModeUAV}>Update</button
	>
	<button class="new-uav" on:click={newUav} disabled={!uavOnEdit || !isEditModeUAV}>New</button>
	<button
		class="delete-uav"
		on:click={deleteUav}
		disabled={uavOnEdit || uavs.length == 0 || !selectedUav || !selectedUav.id || !isEditModeUAV}
		>Delete</button
	>
	<button class="undo-uav" on:click={undoUav} disabled={!uavOnEdit || !isEditModeUAV}>Undo</button>
</div>
