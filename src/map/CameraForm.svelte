<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	interface Camera {
		id: number;
		name: string;
		mass: number;
		fov_x: number;
		fov_y: number;
		resolution_x: number;
		resolution_y: number;
	}

	let cameras: Camera[] = [];
	let selectedCamera: Camera | null = null;

	async function fetchCameras() {
		try {
			const result = await invoke<Camera[]>('get_cameras_vec');
			cameras = result;
			selectedCamera = cameras.length > 0 ? cameras[0] : null;
			console.log(cameras);
		} catch (error) {
			console.error('Failed to fetch cameras:', error);
		}
	}

	let cameraOnEdit = false;

	function onCameraFieldChange() {
		cameraOnEdit = true;
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
		if (camera.fov_x < 1 || camera.fov_x > 180 || camera.fov_y < 1 || camera.fov_y > 180) {
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

	async function updateCamera() {
		const camera: Camera = {
			id: parseInt((document.getElementById('camera_id') as HTMLInputElement).value),
			name: (document.getElementById('camera_name') as HTMLInputElement).value,
			mass: parseInt((document.getElementById('camera_mass') as HTMLInputElement).value),
			fov_x: parseFloat((document.getElementById('camera_fov_x') as HTMLInputElement).value),
			fov_y: parseFloat((document.getElementById('camera_fov_y') as HTMLInputElement).value),
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

				selectedCamera = cameras.length > 0 ? cameras[index] : null;
				cameraOnEdit = false;
			}
		} else {
			console.error('Invalid camera data');
		}
	}

	async function newCamera() {
		const camera: Camera = {
			id: 0,
			name: (document.getElementById('camera_name') as HTMLInputElement).value,
			mass: parseInt((document.getElementById('camera_mass') as HTMLInputElement).value),
			fov_x: parseFloat((document.getElementById('camera_fov_x') as HTMLInputElement).value),
			fov_y: parseFloat((document.getElementById('camera_fov_y') as HTMLInputElement).value),
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
				selectedCamera = cameras.length > 0 ? cameras[cameras.length - 1] : null;
				cameraOnEdit = false;
			}
		} else {
			console.error('Invalid Camera data');
		}
	}

	async function deleteCamera() {
		const camera: Camera = {
			id: parseInt((document.getElementById('camera_id') as HTMLInputElement).value),
			name: 'camera_name',
			mass: 0,
			fov_x: 0,
			fov_y: 0,
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

			selectedCamera = cameras.length > 0 ? cameras[0] : null;
			cameraOnEdit = false;
		}
	}

	function undoCamera() {
		if (cameraOnEdit && selectedCamera) {
			(document.getElementById('camera_name') as HTMLInputElement).value = selectedCamera.name;
			(document.getElementById('camera_mass') as HTMLInputElement).value =
				selectedCamera.mass.toString();
			(document.getElementById('camera_fov_x') as HTMLInputElement).value =
				selectedCamera.fov_x.toString();
			(document.getElementById('camera_fov_y') as HTMLInputElement).value =
				selectedCamera.fov_y.toString();
			(document.getElementById('camera_resolution_x') as HTMLInputElement).value =
				selectedCamera.resolution_x.toString();
			(document.getElementById('camera_resolution_y') as HTMLInputElement).value =
				selectedCamera.resolution_y.toString();
			cameraOnEdit = false;
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

	let isEditModeCamera = false;
	function toggleEditModeCamera() {
		isEditModeCamera = !isEditModeCamera;
	}

	onMount(() => {
		fetchCameras();
	});
</script>

<div class="uav-select-fetch-wrapper">
	<select bind:value={selectedCamera} on:change={() => {}} disabled={cameraOnEdit}>
		{#each cameras as camera (camera.id)}
			<option value={camera}>{camera.name}</option>
		{/each}
	</select>
	<button on:click={fetchCameras} class="fetch-camera">Fetch</button>
</div>

<button on:click={toggleCameraBlock} class="toggle-display">Camera detatils</button>
<div class="block" id="camera">
	<input
		type="checkbox"
		on:change={toggleEditModeCamera}
		class="edit-mode-checkbox"
		disabled={cameraOnEdit}
	/>
	<label for="edit-mode-camera" class="edit-mode-label">Edit Mode</label>

	<div class="parameters">
		<label for="camera_id" class="label">ID:</label>
		<input
			type="number"
			class="input"
			id="camera_id"
			value={selectedCamera ? selectedCamera.id : ''}
			readonly
		/>

		<label for="camera_name" class="label">Name:</label>
		<input
			type="text"
			class="input"
			id="camera_name"
			value={selectedCamera ? selectedCamera.name : ''}
			readonly={!isEditModeCamera}
			on:input={onCameraFieldChange}
		/>

		<label for="camera_mass" class="label">Mass (grams):</label>
		<input
			type="number"
			class="input"
			id="camera_mass"
			value={selectedCamera ? selectedCamera.mass : ''}
			readonly={!isEditModeCamera}
			on:input={onCameraFieldChange}
		/>

		<label for="camera_fov_x" class="label">X-axis FOV (degrees):</label>
		<input
			type="number"
			class="input"
			id="camera_fov_x"
			value={selectedCamera ? selectedCamera.fov_x : ''}
			readonly={!isEditModeCamera}
			on:input={onCameraFieldChange}
		/>

		<label for="camera_fov_y" class="label">Y-axis FOV (degrees):</label>
		<input
			type="number"
			class="input"
			id="camera_fov_y"
			value={selectedCamera ? selectedCamera.fov_y : ''}
			readonly={!isEditModeCamera}
			on:input={onCameraFieldChange}
		/>

		<label for="camera_resolution_x" class="label">Resolution X:</label>
		<input
			type="number"
			class="input"
			id="camera_resolution_x"
			value={selectedCamera ? selectedCamera.resolution_x : ''}
			readonly={!isEditModeCamera}
			on:input={onCameraFieldChange}
		/>

		<label for="camera_resolution_y" class="label">Resolution Y:</label>
		<input
			type="number"
			class="input"
			id="camera_resolution_y"
			value={selectedCamera ? selectedCamera.resolution_y : ''}
			readonly={!isEditModeCamera}
			on:input={onCameraFieldChange}
		/>
	</div>

	<div class="uav-edit-toolbar">
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
			disabled={cameraOnEdit ||
				cameras.length == 0 ||
				!selectedCamera ||
				!selectedCamera.id ||
				!isEditModeCamera}
		>
			Delete
		</button>
		<button class="undo-camera" on:click={undoCamera} disabled={!cameraOnEdit || !isEditModeCamera}>
			Undo
		</button>
	</div>
</div>
