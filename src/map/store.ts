import { writable } from 'svelte/store';
export interface Camera {
	id: number;
	name: string;
	mass: number;
	fov_x: number;
	resolution_x: number;
	resolution_y: number;
}

export interface Uav {
	id: number;
	name: string;
	max_payload_mass: number;
	flight_duration: number;
	takeoff_speed: number;
	flight_speed: number;
	min_altitude: number;
	max_altitude: number;
}


export const selectedUav = writable<Uav | null>(null);
export const selectedCamera = writable<Camera | null>(null);

export const altitudeValue = writable<number>(0);

export const overlapValue = writable<number>(0);

export interface MissionPlan {
    area_vertices: number[][][];
    mission_uav: Uav;
    mission_camera: Camera;
    mission_altitude: number;
    mission_start: number[];
}
 

export enum Algorithm {
  NearestNeighbor = "NearestNeighbor",
  ChristofidesAlgorithm = "ChristofidesAlgorithm",
  BruteForce = "BruteForce",
}

export const selectedAlgorithm = writable<Algorithm>(Algorithm.NearestNeighbor);
