<script lang="ts">
	import { onMount } from 'svelte';

	let isActive = false;
	let sidenavWidth = 0;

	function toggle() {
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

	onMount(() => {});
</script>

<div class="sidenav" style={`width: ${sidenavWidth}px`} >
  <label for="fname">First name:</label>
  <input type="text" id="fname" name="fname"><br><br>
</div>

<div class="toggle-container">
  <button on:click={toggle} class="toggle-button">
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
