use rand::Rng;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

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
        )
    }

    pub fn fly(&self) {
        println!("{} is flying!", self.name);
    }

    pub fn land(&self) {
        println!("{} is landing!", self.name);
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
    }

    pub fn sql_create_table(conn: &Connection) -> Result<usize> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS uav (
                    uav_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                    uav_name TEXT NOT NULL,
                    uav_max_payload_mass INTEGER NOT NULL CHECK (uav_max_payload_mass >= 0),
                    uav_flight_duration INTEGER NOT NULL CHECK (uav_flight_duration >= 0),
                    uav_takeoff_speed REAL NOT NULL CHECK (uav_takeoff_speed >= 0),
                    uav_flight_speed REAL NOT NULL CHECK (uav_flight_speed >= 0),
                    uav_min_altitude REAL DEFAULT 0 NOT NULL CHECK (uav_min_altitude >= 0),
                    uav_max_altitude REAL DEFAULT 0 NOT NULL CHECK (uav_max_altitude >= 0)
                    )",
            (), // empty list of parameters.
        )
    }

    pub fn sql_add_to_db(&self, conn: &Connection) -> Result<usize> {
        conn.execute(
            "INSERT INTO uav (
                    uav_name,
                    uav_max_payload_mass,
                    uav_flight_duration,
                    uav_takeoff_speed,
                    uav_flight_speed,
                    uav_min_altitude,
                    uav_max_altitude
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                &self.name,
                &self.max_payload_mass,
                &self.flight_duration,
                &self.takeoff_speed,
                &self.flight_speed,
                &self.min_altitude,
                &self.max_altitude,
            ),
        )
    }

    pub fn sql_update(&self, conn: &Connection) -> Result<usize> {
        conn.execute(
            "
                    UPDATE uav SET
                        uav_name = ?1,
                        uav_max_payload_mass = ?2,
                        uav_flight_duration = ?3,
                        uav_takeoff_speed = ?4,
                        uav_flight_speed = ?5,
                        uav_min_altitude = ?6,
                        uav_max_altitude = ?7
                    WHERE uav_id = ?8",
            (
                &self.name,
                &self.max_payload_mass,
                &self.flight_duration,
                &self.takeoff_speed,
                &self.flight_speed,
                &self.min_altitude,
                &self.max_altitude,
                &self.id,
            ),
        )
    }

    pub fn sql_delete(&self, conn: &Connection) -> Result<usize> {
        conn.execute(
            "DELETE FROM uav 
            WHERE uav_id = ?1",
            (
                &self.id,
            ),
        )
    }

    pub fn sql_get_uavs(conn: &Connection) -> Result<Vec<Result<Uav>>> {
        let mut stmt = conn.prepare(
            "SELECT
                        uav_id,
                        uav_name,
                        uav_max_payload_mass,
                        uav_flight_duration,
                        uav_takeoff_speed,
                        uav_flight_speed,
                        uav_min_altitude,
                        uav_max_altitude 
                    FROM uav",
        )?;

        let uav_iter = stmt.query_map([], |row| {
            Ok(Uav {
                id: row.get(0)?,
                name: row.get(1)?,
                max_payload_mass: row.get(2)?,
                flight_duration: row.get(3)?,
                takeoff_speed: row.get(4)?,
                flight_speed: row.get(5)?,
                min_altitude: row.get(6)?,
                max_altitude: row.get(7)?,
            })
        })?;

        // let mut uav_vector: Vec<Result<Uav>> = Vec::new();
        // for uav_item in uav_iter {
        //     uav_vector.push(uav_item);
        // }

        // Ok(uav_vector)
        Ok(uav_iter.collect())
    }

    pub fn get_uavs(conn: &Connection) -> Result<Vec<Uav>> {
        let uav_results = Self::sql_get_uavs(conn)?;

        let mut uavs: Vec<Uav> = Vec::new();
        for uav_result in uav_results {
            match uav_result {
                Ok(uav) => uavs.push(uav),
                Err(err) => eprintln!("Error processing a UAV: {}", err),
            }
        }
        Ok(uavs)
    }
}
