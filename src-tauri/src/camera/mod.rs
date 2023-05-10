use rand::Rng;

use serde::{Deserialize, Serialize};
pub mod camera_handle;
pub mod camera_sql;

#[derive(Debug, Deserialize, Serialize)]
pub struct Camera {
    id: u64,               // id
    pub name: String,      // name
    pub mass: u64,         // mass in grams
    pub fov_x: f64,        // x-axis viewing angle in degrees
    pub resolution_x: u16, // camera resolution x
    pub resolution_y: u16, // camera resolution y
}

impl Camera {
    pub fn new(
        name: String,
        mass: u64,
        fov_x: f64,
        resolution_x: u16,
        resolution_y: u16,
    ) -> Camera {
        Camera {
            id: 0,
            name,
            mass,
            fov_x,
            resolution_x,
            resolution_y,
        }
    }

    pub fn new_random() -> Camera {
        let mut rng = rand::thread_rng();
        let name = format!("Fake Camera {}", rng.gen_range(1..100));
        let mass = rng.gen_range(100..1000);
        let fov_x = rng.gen_range(30.0..180.0);
        let resolution_x = rng.gen_range(1000..6000);
        let resolution_y = rng.gen_range(800..4000);

        Camera::new(name, mass, fov_x, resolution_x, resolution_y)
    }

    pub fn print_camera(&self) {
        println!("id: {}", &self.id);
        println!("name: {}", &self.name);
        println!("mass: {}", &self.mass);
        println!("fov_x: {}", &self.fov_x);
        println!("resolution_x: {}", &self.resolution_x);
        println!("resolution_y: {}", &self.resolution_y);
    }

    
}
