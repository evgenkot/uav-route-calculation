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
  <!-- add a close button to the top right corner -->
  <!-- <a href="#" class="closebtn" on:click={closeNav}>&times;</a> -->
  <!-- add menu links -->
  <!-- <a href="#">Link 1</a>
  <a href="#">Link 2</a>
  <a href="#">Link 3</a> -->
</div>

<!-- move the button inside a container div -->
<div class="toggle-container">
  <button on:click={toggle} class="toggle-button">
    Details {isActive ? 'Hide' : 'Show'}
  </button>
</div>

<style>
  .sidenav {
    /* set position to fixed and right to 0 */
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
  
  .sidenav a {
    /* remove left padding to align with the right edge */
    padding: 8px 8px 8px 0;
    /* add text-align right to align text with the right edge */
    text-align: right;
    text-decoration: none;
    font-size: 25px;
    color: #818181;
    display: block;
    transition: 0.3s;
  }
  
  .sidenav a:hover {
    color: #f1f1f1;
  }
  
  .sidenav .closebtn {
    position: absolute;
    top: 0;
    right: 25px;
    font-size: 36px;
    margin-left: 50px;
    z-index: 1;
  }
  
  /* add styles for the toggle button container */
  .toggle-container {
    position: absolute;
    top: 0;
    right: 0;
    padding: 16px;
    z-index: 2;
  }
  
  /* add styles for the toggle button */
  .toggle-button {
    /* background-color: transparent; */
    /* border: none; */
    /* color: white; */
    font-size: 20px;
    cursor: pointer;
  }
  
  /* adjust media query to account for the toggle button */
  @media screen and (max-height: 450px), screen and (max-width: 600px) {
    .sidenav {
      padding-top: 15px;
      /* reduce width to account for toggle button */
      width: 150px;
    }
  
    .sidenav a {
      font-size: 18px;
    }
    
    /* adjust toggle button container padding */
    .toggle-container {
      padding: 8px;
    }
  }
</style>
