<script lang="ts">
	import { onMount } from 'svelte';

	import { invoke } from '@tauri-apps/api/tauri';
	import type { Camera, Uav } from './store';
	import { selectedCamera, selectedUav } from './store';

	let uavs: Uav[] = [];
	let cameras: Camera[] = [];

	let uavOnEdit = false;
	let cameraOnEdit = false;

	function onUavFieldChange() {
		uavOnEdit = true;
	}

	function onCameraFieldChange() {
		cameraOnEdit = true;
	}

	async function fetchUavs() {
		try {
			const result = await invoke<Uav[]>('get_all_uavs_vec');
			uavs = result;
			selectedUav.set(uavs.length > 0 ? uavs[0] : null);
			fetchCameras();
			console.log(uavs);
		} catch (error) {
			console.error('Failed to fetch UAVs:', error);
		}
	}

	function setUav() {
		console.log("selected Uav:", $selectedUav);
		fetchCameras();
	}

	function setProperCamera() {
		selectedCamera.set(
			$selectedUav?.camera_id !== null
				? cameras.find((camera) => camera.id === $selectedUav?.camera_id) || null
				: null
		);
	}

	async function fetchCameras() {
		const currentUavId = $selectedUav?.id;
		try {
			const result = await invoke<Camera[]>('get_all_cameras_vec');
			const uavCameraIds = uavs
				.map((mapUav) => mapUav.camera_id)
				.filter((filterUavId) => filterUavId != currentUavId);

			cameras = result.filter((camera) => !uavCameraIds.includes(camera.id));

			selectedCamera.set(cameras.length > 0 ? cameras[0] : null);
			setProperCamera();
		} catch (error) {
			console.error('Failed to fetch cameras:', error);
		}
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

		// Check if camera is already set to another UAV
		if ($selectedUav?.camera_id != null) {
			for (const current_uav of uavs) {
				if ((current_uav.camera_id = uav.camera_id)) {					
					alert('The camera is already mounted on another UAV');
					return false;
				}
			}
		}

		return true;
	}

	function isCameraValid(camera: Camera): boolean {
		// Check if the camera name is not empty
		if (!camera.name || camera.name.trim() === '') {
			alert('Camera name cannot be empty');
			return false;
		}

		// Check if the mass is within a reasonable range (e.g., 10g to 100no00g)
		if (camera.mass < 10 || camera.mass > 10000) {
			alert('Mass should be between 10g and 10000g');
			return false;
		}

		// Check if the viewing angles are within a reasonable range (e.g., 1° to 180°)
		if (camera.fov_x < 1 || camera.fov_x > 180) {
			alert('Viewing angles should be between 1° and 180°');
			return false;
		}

		// Check if the resolutions are within a reasonable range (e.g., 1 to 10000)
		if (
			camera.resolution_x < 1 ||
			camera.resolution_x > 10000 ||
			camera.resolution_y < 1 ||
			camera.resolution_y > 10000
		) {
			alert('Resolutions should be between 1 and 10000');
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
			),

			camera_id: $selectedCamera?.id ?? null
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

				selectedUav.set(uavs.length > 0 ? uavs[index] : null);
				uavOnEdit = false;
			}
		} else {
			console.error('Invalid UAV data');
		}
	}

	async function updateCamera() {
		const camera: Camera = {
			id: parseInt((document.getElementById('camera_id') as HTMLInputElement).value),
			name: (document.getElementById('camera_name') as HTMLInputElement).value,
			mass: parseInt((document.getElementById('camera_mass') as HTMLInputElement).value),
			fov_x: parseFloat((document.getElementById('camera_fov_x') as HTMLInputElement).value),
			resolution_x: parseInt(
				(document.getElementById('camera_resolution_x') as HTMLInputElement).value
			),
			resolution_y: parseInt(
				(document.getElementById('camera_resolution_y') as HTMLInputElement).value
			)
		};
		if (isCameraValid(camera)) {
			let response = await invoke<string>('update_camera', { camera });
			if (response != 'Ok') {
				alert(response);
			} else {
				// Find the index of the camera in the local list with the same ID
				const index = cameras.findIndex((item) => item.id === camera.id);

				// Update the local camera list
				if (index !== -1) {
					cameras[index] = camera;
					cameras = [...cameras]; // Trigger reactivity by creating a new array reference
				}

				selectedCamera.set(cameras.length > 0 ? cameras[index] : null);
				cameraOnEdit = false;
			}
		} else {
			console.error('Invalid camera data');
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
			),
			camera_id: $selectedCamera?.id ?? null
		};

		if (isUavValid(uav)) {
			let response = await invoke<string>('new_uav', { uav }); // Update the payload here
			if (response != 'Ok') {
				alert(response);
			} else {
				await fetchUavs();
				selectedUav.set(uavs.length > 0 ? uavs[uavs.length - 1] : null);
				uavOnEdit = false;
			}
		} else {
			console.error('Invalid UAV data');
		}
	}

	async function newCamera() {
		const camera: Camera = {
			id: 0,
			name: (document.getElementById('camera_name') as HTMLInputElement).value,
			mass: parseInt((document.getElementById('camera_mass') as HTMLInputElement).value),
			fov_x: parseFloat((document.getElementById('camera_fov_x') as HTMLInputElement).value),
			resolution_x: parseInt(
				(document.getElementById('camera_resolution_x') as HTMLInputElement).value
			),
			resolution_y: parseInt(
				(document.getElementById('camera_resolution_y') as HTMLInputElement).value
			)
		};

		if (isCameraValid(camera)) {
			let response = await invoke<string>('new_camera', { camera });
			if (response != 'Ok') {
				alert(response);
			} else {
				await fetchCameras();
				selectedCamera.set(cameras.length > 0 ? cameras[cameras.length - 1] : null);
				cameraOnEdit = false;
				onUavFieldChange();
			}
		} else {
			console.error('Invalid Camera data');
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
			max_altitude: 0,
			camera_id: 0
		};
		let response = await invoke<string>('delete_uav', { uav });
		if (response != 'Ok') {
			alert(response);
		} else {
			const index = uavs.findIndex((item) => item.id === uav.id);

			// Update the local UAV list
			fetchUavs();
			fetchCameras();

			selectedUav.set(uavs.length > 0 ? uavs[0] : null);
			uavOnEdit = false;
		}
	}

	async function deleteCamera() {
		const camera: Camera = {
			id: parseInt((document.getElementById('camera_id') as HTMLInputElement).value),
			name: 'camera_name',
			mass: 0,
			fov_x: 0,
			resolution_x: 0,
			resolution_y: 0
		};
		let response = await invoke<string>('delete_camera', { camera });
		if (response != 'Ok') {
			alert(response);
		} else {
			const index = cameras.findIndex((item) => item.id === camera.id);

			// Update the local camera list
			fetchCameras();

			selectedCamera.set(cameras.length > 0 ? cameras[0] : null);
			cameraOnEdit = false;
		}
	}

	function undoUav() {
		if (uavOnEdit && selectedUav) {
			(document.getElementById('uav_name') as HTMLInputElement).value = $selectedUav?.name || '';
			(document.getElementById('uav_max_payload_mass') as HTMLInputElement).value =
				$selectedUav?.max_payload_mass.toString() || '';
			(document.getElementById('uav_flight_duration') as HTMLInputElement).value =
				$selectedUav?.flight_duration.toString() || '';
			(document.getElementById('uav_takeoff_speed') as HTMLInputElement).value =
				$selectedUav?.takeoff_speed.toString() || '';
			(document.getElementById('uav_flight_speed') as HTMLInputElement).value =
				$selectedUav?.flight_speed.toString() || '';
			(document.getElementById('uav_min_altitude') as HTMLInputElement).value =
				$selectedUav?.min_altitude.toString() || '';
			(document.getElementById('uav_max_altitude') as HTMLInputElement).value =
				$selectedUav?.max_altitude.toString() || '';
			setProperCamera();
			uavOnEdit = false;
		}
	}

	function undoCamera() {
		if (cameraOnEdit && selectedCamera) {
			(document.getElementById('camera_name') as HTMLInputElement).value =
				$selectedCamera?.name || '';

			(document.getElementById('camera_mass') as HTMLInputElement).value =
				$selectedCamera?.mass.toString() || '';
			(document.getElementById('camera_fov_x') as HTMLInputElement).value =
				$selectedCamera?.fov_x.toString() || '';
			(document.getElementById('camera_resolution_x') as HTMLInputElement).value =
				$selectedCamera?.resolution_x.toString() || '';
			(document.getElementById('camera_resolution_y') as HTMLInputElement).value =
				$selectedCamera?.resolution_y.toString() || '';
			cameraOnEdit = false;
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

	let isEditModeUAV = false;
	function toggleEditModeUAV() {
		isEditModeUAV = !isEditModeUAV;
	}

	let isEditModeCamera = false;
	function toggleEditModeCamera() {
		isEditModeCamera = !isEditModeCamera;
	}

	onMount(() => {
		fetchUavs();
	});
</script>

<div class="uav-select-fetch-wrapper">
	<select bind:value={$selectedUav} on:change={setUav} disabled={uavOnEdit || isEditModeUAV}>
		{#each uavs as uav (uav.id)}
			<option value={uav}>{uav.name}</option>
		{/each}
	</select>
	<button on:click={fetchUavs} class="fetch-uav">Fetch</button>
</div>

<button on:click={toggleUAVBlock} class="toggle-display">UAV detatils</button>
<div class="block" id="uav">
	<input
		type="checkbox"
		on:change={toggleEditModeUAV}
		class="edit-mode-checkbox"
		disabled={uavOnEdit || isEditModeCamera}
	/>
	<label for="edit-mode-uav" class="edit-mode-label">Edit Mode</label>
	<div class="parameters">
		<label for="uav_id" class="label">ID:</label>
		<input
			type="number"
			class="input"
			id="uav_id"
			value={$selectedUav ? $selectedUav.id : ''}
			readonly
		/>

		<label for="uav_name" class="label">Name:</label>
		<input
			type="text"
			class="input"
			id="uav_name"
			value={$selectedUav ? $selectedUav.name : ''}
			readonly={!isEditModeUAV}
			on:input={onUavFieldChange}
		/>

		<label for="uav_max_payload_mass" class="label">Max Payload Mass:</label>
		<input
			type="number"
			class="input"
			id="uav_max_payload_mass"
			value={$selectedUav ? $selectedUav.max_payload_mass : ''}
			readonly={!isEditModeUAV}
			on:input={onUavFieldChange}
		/>

		<label for="uav_flight_duration" class="label">Flight Duration:</label>
		<input
			type="number"
			class="input"
			id="uav_flight_duration"
			value={$selectedUav ? $selectedUav.flight_duration : ''}
			readonly={!isEditModeUAV}
			on:input={onUavFieldChange}
		/>

		<label for="takeoff_speed" class="label">Takeoff speed:</label>
		<input
			type="number"
			class="input"
			id="uav_takeoff_speed"
			value={$selectedUav ? $selectedUav.takeoff_speed : ''}
			readonly={!isEditModeUAV}
			on:input={onUavFieldChange}
		/>

		<label for="flight_speed" class="label">Flight speed:</label>
		<input
			type="number"
			class="input"
			id="uav_flight_speed"
			value={$selectedUav ? $selectedUav.flight_speed : ''}
			readonly={!isEditModeUAV}
			on:input={onUavFieldChange}
		/>

		<label for="min_altitude" class="label">Min altitude:</label>
		<input
			type="number"
			class="input"
			id="uav_min_altitude"
			value={$selectedUav ? $selectedUav.min_altitude : ''}
			readonly={!isEditModeUAV}
			on:input={onUavFieldChange}
		/>

		<label for="max_altitude" class="label">Max altitude:</label>
		<input
			type="number"
			class="input"
			id="uav_max_altitude"
			value={$selectedUav ? $selectedUav.max_altitude : ''}
			readonly={!isEditModeUAV}
			on:input={onUavFieldChange}
		/>
	</div>

	<div class="camera-select-fetch-wrapper">
		<select
			bind:value={$selectedCamera}
			on:change={onUavFieldChange}
			disabled={cameraOnEdit || isEditModeCamera || !isEditModeUAV}
		>
			<option value={null}>None</option>
			{#each cameras as camera (camera.id)}
				<option value={camera}>{camera.name}</option>
			{/each}
		</select>
		<button on:click={fetchCameras} class="fetch-camera">Fetch</button>
	</div>

	<button on:click={toggleCameraBlock} class="toggle-display">Camera detatils</button>
	<div class="block" id="camera" style="display:none;">
		<input
			type="checkbox"
			on:change={toggleEditModeCamera}
			class="edit-mode-checkbox"
			disabled={cameraOnEdit || !isEditModeUAV}
		/>
		<label for="edit-mode-camera" class="edit-mode-label">Edit Mode</label>

		<div class="parameters">
			<label for="camera_id" class="label">ID:</label>
			<input
				type="number"
				class="input"
				id="camera_id"
				value={$selectedCamera ? $selectedCamera.id : ''}
				readonly
			/>

			<label for="camera_name" class="label">Name:</label>
			<input
				type="text"
				class="input"
				id="camera_name"
				value={$selectedCamera ? $selectedCamera.name : ''}
				readonly={!isEditModeCamera}
				on:input={onCameraFieldChange}
			/>

			<label for="camera_mass" class="label">Mass (grams):</label>
			<input
				type="number"
				class="input"
				id="camera_mass"
				value={$selectedCamera ? $selectedCamera.mass : ''}
				readonly={!isEditModeCamera}
				on:input={onCameraFieldChange}
			/>

			<label for="camera_fov_x" class="label">X-axis FOV (degrees):</label>
			<input
				type="number"
				class="input"
				id="camera_fov_x"
				value={$selectedCamera ? $selectedCamera.fov_x : ''}
				readonly={!isEditModeCamera}
				on:input={onCameraFieldChange}
			/>

			<label for="camera_resolution_x" class="label">Resolution X:</label>
			<input
				type="number"
				class="input"
				id="camera_resolution_x"
				value={$selectedCamera ? $selectedCamera.resolution_x : ''}
				readonly={!isEditModeCamera}
				on:input={onCameraFieldChange}
			/>

			<label for="camera_resolution_y" class="label">Resolution Y:</label>
			<input
				type="number"
				class="input"
				id="camera_resolution_y"
				value={$selectedCamera ? $selectedCamera.resolution_y : ''}
				readonly={!isEditModeCamera}
				on:input={onCameraFieldChange}
			/>
		</div>

		<div class="camera-edit-toolbar">
			<button
				class="update-camera"
				on:click={updateCamera}
				disabled={!cameraOnEdit || cameras.length == 0 || !isEditModeCamera}
			>
				Update
			</button>
			<button class="new-camera" on:click={newCamera} disabled={!cameraOnEdit || !isEditModeCamera}>
				New
			</button>
			<button
				class="delete-camera"
				on:click={deleteCamera}
				disabled={cameraOnEdit || cameras.length == 0 || !$selectedCamera || !isEditModeCamera}
			>
				Delete
			</button>
			<button
				class="undo-camera"
				on:click={undoCamera}
				disabled={!cameraOnEdit || !isEditModeCamera}
			>
				Undo
			</button>
		</div>
	</div>

	<div class="uav-edit-toolbar">
		<button
			class="update-uav"
			on:click={updateUav}
			disabled={!uavOnEdit || uavs.length == 0 || !isEditModeUAV || isEditModeCamera}>Update</button
		>
		<button
			class="new-uav"
			on:click={newUav}
			disabled={!uavOnEdit || !isEditModeUAV || isEditModeCamera}>New</button
		>
		<button
			class="delete-uav"
			on:click={deleteUav}
			disabled={uavOnEdit ||
				uavs.length == 0 ||
				!$selectedUav ||
				!isEditModeUAV ||
				isEditModeCamera}>Delete</button
		>
		<button
			class="undo-uav"
			on:click={undoUav}
			disabled={!uavOnEdit || !isEditModeUAV || isEditModeCamera}>Undo</button
		>
	</div>
</div>
