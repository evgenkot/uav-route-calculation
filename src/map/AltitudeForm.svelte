<script lang="ts">
    import { writable } from "svelte/store";

    let altitudeMode = "manual";
    const altitudeValue = writable<number>(0);
    const pxPerMeterValue = writable<number>(0);

    function onAltitudeModeChange(event: Event) {
        altitudeMode = (event.target as HTMLInputElement).value;
    }

    function onAltitudeValueChange(event: Event) {
        altitudeValue.set(parseFloat((event.target as HTMLInputElement).value));
    }

    function onPxPerMeterValueChange(event: Event) {
        pxPerMeterValue.set(parseFloat((event.target as HTMLInputElement).value));
    }
</script>

<style>
    .menu {
        display: flex;
        flex-direction: column;
    }

    .input-row {
        display: flex;
        align-items: center;
    }

    .input-row:not(:first-child) {
        margin-top: 8px;
    }

    .input-row input[type="number"] {
        margin-left: 8px;
    }
</style>

<div class="menu">
    <div class="input-row">
        <input
            type="radio"
            id="manual-altitude"
            name="altitude-mode"
            value="manual"
            checked={altitudeMode === "manual"}
            on:change={onAltitudeModeChange}
        />
        <label for="manual-altitude">Manual altitude input</label>
    </div>
    <div class="input-row">
        <input
            type="radio"
            id="calculate-px-m"
            name="altitude-mode"
            value="calculate"
            checked={altitudeMode === "calculate"}
            on:change={onAltitudeModeChange}
        />
        <label for="calculate-px-m">Calculate using px/m</label>
    </div>
    <div class="input-row">
        <label for="altitude">Altitude:</label>
        <input
            type="number"
            id="altitude"
            min="0"
            step="0.01"
            disabled={altitudeMode === "calculate"}
            on:input={onAltitudeValueChange}
        />
    </div>
    {#if altitudeMode === "calculate"}
    <div class="input-row">
        <label for="px-per-meter">Px/m:</label>
        <input
            type="number"
            id="px-per-meter"
            min="0"
            step="0.01"
            on:input={onPxPerMeterValueChange}
        />
    </div>
    {/if}
</div>
