<script lang="ts">
	import { onMount } from 'svelte';
	import UAVForm from './UAVForm.svelte';
	import CameraForm from './CameraForm.svelte';


	import { writable } from 'svelte/store';
	import { invoke } from '@tauri-apps/api/tauri';

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
		// fetchUavs();
	});
</script>

<div class="sidenav" style={`width: ${sidenavWidth}px`}>
<!-- <div style={padding: 60px} ></div> -->

		
	<h1 style={`padding-top: 40px`}>Menu</h1>
	<UAVForm/>
	<CameraForm/>
	<!-- <button on:click={toggleCameraBlock} class="toggle-display">Camera detatils</button>
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
	</div> -->

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
