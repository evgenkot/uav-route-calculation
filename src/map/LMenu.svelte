<script lang="ts">
	import { onMount } from 'svelte';
	import UAVForm from './UAVForm.svelte';
	import CameraForm from './CameraForm.svelte';

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

	onMount(() => {
		
	});
</script>

<div class="sidenav" style={`width: ${sidenavWidth}px`}>

		
	<h1 style={`padding-top: 40px`}>Menu</h1>
	<UAVForm/>
	<CameraForm/>
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
