<script lang="ts">
	import { onMount } from 'svelte';

	let isActive = false;
	let sidenavWidth = 0;

	function toggleMenu() {
		isActive = !isActive;
		if (isActive) {
			sidenavWidth = 250;
		} else {
			sidenavWidth = 0;
		}
	}

	function openNav() {
		sidenavWidth = 250;
	}

	function closeNav() {
		sidenavWidth = 0;
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

	onMount(() => {});
</script>

<div class="sidenav" style={`width: ${sidenavWidth}px`}>
	<h1>Menu</h1>
	<button on:click={toggleUAVBlock} class="toggle-display">UAV detatils</button>
	<div class="block" id="uav">
		<div class="parameters">
			<label for="uav_id" class="label">ID:</label>
			<input type="text" class="input" id="uav_id" />
		</div>
		<div class="parameters">
			<label for="uav_name" class="label">Name:</label>
			<input type="text" class="input" id="uav_name" />
		</div>
		<div class="parameters">
			<label for="uav_max_payload_mass" class="label">Max Payload Mass:</label>
			<input type="text" class="input" id="uav_max_payload_mass" />
		</div>
		<div class="parameters">
			<label for="uav_flight_duration" class="label">Flight Duration:</label>
			<input type="text" class="input" id="uav_flight_duration" />
		</div>
		<div class="parameters">
			<label for="takeoff_speed" class="label">Takeoff speed:</label>
			<input type="text" class="input" id="takeoff_speed" />
		</div>
		<div class="parameters">
			<label for="flight_speed" class="label">Flight speed:</label>
			<input type="text" class="input" id="flight_speed" />
		</div>
		<div class="parameters">
			<label for="min_altitude" class="label">Min altitude:</label>
			<input type="text" class="input" id="min_altitude" />
		</div>
		<div class="parameters">
			<label for="max_altitude" class="label">Max altitude:</label>
			<input type="text" class="input" id="max_altitude" />
		</div>
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
	.sidenav {
		position: fixed;
		top: 0;
		left: 0;
		height: 100%;
		width: 0;
		z-index: 1;
		background-color: #ffffff;
		overflow-x: hidden;
		transition: 0.5s;
		padding-top: 60px;
	}

	.toggle-container {
		position: absolute;
		top: 0;
		left: 0;
		padding: 16px;
		z-index: 2;
	}
	.toggle-button {
		font-size: 20px;
		cursor: pointer;
	}

	@media screen and (max-height: 450px), screen and (max-width: 600px) {
		.sidenav {
			padding-top: 15px;
			width: 150px;
		}
		.toggle-container {
			padding: 8px;
		}
	}

	.block {
		margin-bottom: 20px;
	}
	.parameters {
		margin-left: 20px;
		margin-bottom: 10px;
	}
	.label {
		display: inline-block;
		width: 150px;
		font-weight: bold;
		margin-right: 10px;
	}
	.input {
		width: 200px;
		padding: 5px;
		margin-right: 10px;
		border-radius: 3px;
		border: 1px solid #ccc;
	}
</style>
