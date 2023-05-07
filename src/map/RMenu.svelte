<script lang="ts">
	import { onMount } from 'svelte';

	import { invoke } from '@tauri-apps/api/tauri';
	import AltitudeForm from './AltitudeForm.svelte';
	import AlgorithmSelect from './AlgorithmSelect.svelte';
	import Mission from './Mission.svelte';
	import { selectedUav, selectedCamera } from './store';

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

	function displaySelectedUavAndCamera() {
		$selectedUav ? console.log('Selected UAV:', $selectedUav) : console.log('No UAV selected');
		$selectedCamera
			? console.log('Selected Camera:', $selectedCamera)
			: console.log('No Camera selected');
	}
	onMount(() => {});
</script>

<div class="sidenav" style={`width: ${sidenavWidth}px`}>
	<h1 style={`padding-top: 40px`}>Menu</h1>
	<button on:click={displaySelectedUavAndCamera}> Display Selected UAV and Camera </button>

	<AltitudeForm />
	<AlgorithmSelect />
	<Mission />
	
</div>

<div class="toggle-container">
	<button on:click={toggleMenu} class="toggle-button">
		Details {isActive ? 'Hide' : 'Show'}
	</button>
</div>

<style>
	@import '../styles/side-menu.css';
	.sidenav {
		right: 0;
	}
	.toggle-container {
		right: 0;
	}
</style>
