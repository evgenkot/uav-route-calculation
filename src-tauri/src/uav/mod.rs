use rand::Rng;
use serde::{Deserialize, Serialize};
pub mod uav_handle;
pub mod uav_sql;

#[derive(Debug, Deserialize, Serialize)]
pub struct Uav {
    id: u64,                   // uav id
    pub name: String,          // uav name
    pub max_payload_mass: u64, // maximum payload in grams
    pub flight_duration: u64,  // average flight duration in seconds
    pub takeoff_speed: f64,    // average takeoff speed in meters per second
    pub flight_speed: f64,     // average flight speed in meters per second
    pub min_altitude: f64,     // minimum safe flight altitude in meters
    pub max_altitude: f64,     // maximum safe flight altitude in meters
    pub camera_id: Option<u64>,     // id of the camera installed on the uav
}

impl Uav {
    pub fn new(
        name: String,
        max_payload_mass: u64,
        flight_duration: u64,
        takeoff_speed: f64,
        flight_speed: f64,
        min_altitude: f64,
        max_altitude: f64,
        camera_id: Option<u64>,
    ) -> Uav {
        Uav {
            id: 0,
            name,
            max_payload_mass,
            flight_duration,
            takeoff_speed,
            flight_speed,
            min_altitude,
            max_altitude,
            camera_id,
        }
    }

    pub fn new_random() -> Uav {
        let mut rng = rand::thread_rng();
        let name = format!("Fake Drone {}", rng.gen_range(1..100));
        let max_payload_mass = rng.gen_range(500..2000);
        let flight_duration = rng.gen_range(10 * 60..30 * 60);
        let takeoff_speed = rng.gen_range(5.0..15.0);
        let flight_speed = rng.gen_range(30.0..60.0);
        let min_altitude = rng.gen_range(10.0..50.0);
        let max_altitude = rng.gen_range(100.0..500.0);

        Uav::new(
            name,
            max_payload_mass,
            flight_duration,
            takeoff_speed,
            flight_speed,
            min_altitude,
            max_altitude,
            None,
        )
    }

    pub fn print_uav(&self) {
        println!("id: {}", &self.id);
        println!("name: {}", &self.name);
        println!("max_payload_mass: {}", &self.max_payload_mass);
        println!("flight_duration: {}", &self.flight_duration);
        println!("takeoff_speed: {}", &self.takeoff_speed);
        println!("flight_speed: {}", &self.flight_speed);
        println!("min_altitude: {}", &self.min_altitude);
        println!("max_altitude: {}", &self.max_altitude);
        match &self.camera_id {
            Some(id) => println!("camera_id: {}", id),
            None => println!("camera_id: None"),
        }
    }
}
