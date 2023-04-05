use rand::Rng;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

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

    pub fn take_picture(&self) {
        println!("{} Click!", self.name);
    }

    pub fn print_camera(&self) {
        println!("id: {}", &self.id);
        println!("name: {}", &self.name);
        println!("mass: {}", &self.mass);
        println!("fov_x: {}", &self.fov_x);
        println!("resolution_x: {}", &self.resolution_x);
        println!("resolution_y: {}", &self.resolution_y);
    }

    pub fn sql_create_table(conn: &Connection) -> Result<usize> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS camera (
                    camera_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                    camera_name TEXT NOT NULL,
                    camera_mass INTEGER NOT NULL CHECK (camera_mass >= 0),
                    camera_fov_x REAL NOT NULL CHECK (
                        camera_fov_x >= 0
                        AND camera_fov_x <= 180
                    ),
                    camera_resolution_x INTEGER NOT NULL CHECK (camera_resolution_x >= 0),
                    camera_resolution_y INTEGER NOT NULL CHECK (camera_resolution_y >= 0)
                    )",
            (),
        )
    }

    pub fn sql_add_to_db(&self, conn: &Connection) -> Result<usize> {
        conn.execute(
            "INSERT INTO camera (
                    camera_name,
                    camera_mass,
                    camera_fov_x,
                    camera_resolution_x,
                    camera_resolution_y
                ) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                &self.name,
                &self.mass,
                &self.fov_x,
                &self.resolution_x,
                &self.resolution_y,
            ),
        )
    }

    pub fn sql_update(&self, conn: &Connection) -> Result<usize> {
        conn.execute(
            "
                    UPDATE camera SET
                        camera_name = ?1,
                        camera_mass = ?2,
                        camera_fov_x = ?3,
                        camera_resolution_x = ?4,
                        camera_resolution_y = ?5
                    WHERE camera_id = ?6",
            (
                &self.name,
                &self.mass,
                &self.fov_x,
                &self.resolution_x,
                &self.resolution_y,
                &self.id,
            ),
        )
    }

    pub fn sql_delete(&self, conn: &Connection) -> Result<usize> {
        conn.execute(
            "DELETE FROM camera 
            WHERE camera_id = ?1",
            (
                &self.id,
            ),
        )
    }

    pub fn sql_get_cameras(conn: &Connection) -> Result<Vec<Result<Camera>>> {
        let mut stmt = conn.prepare(
            "SELECT
                    camera_id,
                    camera_name,
                    camera_mass,
                    camera_fov_x,
                    camera_resolution_x,
                    camera_resolution_y
                FROM camera",
        )?;

        let camera_iter = stmt.query_map([], |row| {
            Ok(Camera {
                id: row.get(0)?,
                name: row.get(1)?,
                mass: row.get(2)?,
                fov_x: row.get(3)?,
                resolution_x: row.get(4)?,
                resolution_y: row.get(5)?,
            })
        })?;

        // let mut camera_vector: Vec<Result<Camera>> = Vec::new();
        // for camera_item in camera_iter {
        //     camera_vector.push(camera_item);
        // }

        // Ok(camera_vector)
        Ok(camera_iter.collect())
    }
    pub fn get_cameras(conn: &Connection) -> Result<Vec<Camera>> {
        let camera_results = Self::sql_get_cameras(conn)?;
    
        let mut cameras: Vec<Camera> = Vec::new();
        for camera_result in camera_results {
            match camera_result{
                Ok(camera) =>cameras.push(camera),
                Err(err) => eprintln!("Error processing a Camera: {}", err),
            }   
        }
        Ok(cameras)
    }
}
