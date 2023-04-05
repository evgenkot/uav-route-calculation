<script lang="ts">
	import { onMount } from 'svelte';

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
		
	});
</script>

<div class="sidenav" style={`width: ${sidenavWidth}px`}>

		
	<h1 style={`padding-top: 40px`}>Menu</h1>

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
  @import "../styles/side-menu.css";
  .sidenav {
		right: 0;
	}
	.toggle-container {
		right: 0;
	}
</style>
