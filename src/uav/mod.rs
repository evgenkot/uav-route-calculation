pub mod uav_mod {
    use rusqlite::{Connection, Result};

    #[derive(Debug)]
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

        pub fn fly(&self) {
            println!("{} is flying!", self.name);
        }

        pub fn land(&self) {
            println!("{} is landing!", self.name);
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


            let mut uav_vector: Vec<Result<Uav>> = Vec::new();
            for uav_item in uav_iter {
                uav_vector.push(uav_item);
            }
        
            Ok(uav_vector)
        }
    }
}
