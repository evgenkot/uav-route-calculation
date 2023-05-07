<script lang="ts">
    import { writable } from 'svelte/store';
    import { routeLength, missionDuration, photoCount } from './store';
  
    let routeLengthValue: number;
    let missionDurationValue: number;
    let photoCountValue: number;
  
    routeLength.subscribe((value) => {
      routeLengthValue = value;
    });

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
      <li>Route Length: {routeLengthValue}</li>
      <li>Mission Duration: {missionDurationValue}</li>
      <li>Number of Photos: {photoCountValue}</li>      
    </ul>
    <button on:click={exportToGeoJSON}>Export to GeoJSON</button>
  </div>
  