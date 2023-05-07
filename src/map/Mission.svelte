<script lang="ts">
    import { writable } from 'svelte/store';
    import { missionDuration, photoCount } from './store';
  
    let missionDurationValue: number;
    let photoCountValue: number;
  
    missionDuration.subscribe((value) => {
      missionDurationValue = value;
    });
  
    photoCount.subscribe((value) => {
      photoCountValue = value;
    });
  
    function exportToGeoJSON() {
        console.log("Export...")
    }

    
  </script>
  
  <div>
    <h2>Mission Parameters</h2>
    <ul>
      <li>Mission Duration: {missionDurationValue}</li>
      <li>Number of Photos: {photoCountValue}</li>
    </ul>
    <button on:click={exportToGeoJSON}>Export to GeoJSON</button>
  </div>
  