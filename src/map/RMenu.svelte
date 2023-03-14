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
  .sidenav {
    position: fixed;
    top: 0;
    right: 0;
    height: 100%;
    width: 0;
    z-index: 1;
    background-color: #111;
    overflow-x: hidden;
    transition: 0.5s;
    padding-top: 60px;
  }
  
  .toggle-container {
    position: absolute;
    top: 0;
    right: 0;
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
</style>
