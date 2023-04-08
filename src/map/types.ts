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
