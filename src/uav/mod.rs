pub mod uav_mod {
    use rusqlite::{Connection, Result};

    #[derive(Debug)]
    pub struct Uav {
        id: u64,
        pub name: String,
        pub max_payload_mass: u64,
        pub flight_duration: u64,
        pub takeoff_speed: f64,
        pub flight_speed: f64,
        pub minimum_altitude: f64,
        pub max_altitude: f64,
    }

    impl Uav {
        pub fn new(
            name: String,
            max_payload_mass: u64,
            flight_duration: u64,
            takeoff_speed: f64,
            flight_speed: f64,
            minimum_altitude: f64,
            max_altitude: f64,
        ) -> Uav {
            Uav {
                id: 0,
                name,
                max_payload_mass,
                flight_duration,
                takeoff_speed,
                flight_speed,
                minimum_altitude,
                max_altitude,
            }
        }

        pub fn fly(&self) {
            println!("{} is flying!", self.name);
        }

        pub fn land(&self) {
            println!("{} is landing!", self.name);
        }
        pub fn sql_create_table(conn: &Connection) {
            conn.execute(
                "CREATE TABLE uav (
                    uav_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                    uav_name TEXT NOT NULL,
                    uav_max_payload_mass INTEGER NOT NULL CHECK (uav_max_payload_mass >= 0),
                    uav_flight_duration INTEGER NOT NULL CHECK (uav_flight_duration >= 0),
                    uav_takeoff_speed DECIMAL(4, 3) NOT NULL CHECK (uav_takeoff_speed >= 0),
                    uav_flight_speed DECIMAL(4, 3) NOT NULL CHECK (uav_flight_speed >= 0),
                    uav_min_altitude DECIMAL(4, 3) DEFAULT 0 NOT NULL CHECK (uav_min_altitude >= 0),
                    uav_max_altitude DECIMAL(4, 3) DEFAULT 0 NOT NULL CHECK (uav_max_altitude >= 0)
                    )",
                (), // empty list of parameters.
            )
            .unwrap();
        }

        pub fn sql_add_to_db(&self, conn: &Connection) -> Result<()> {
            
            
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
                    &self.minimum_altitude,
                    &self.max_altitude,
                ),
            )
            .unwrap();
            Ok(())
        }

        pub fn sql_get_uavs(conn: &Connection) -> Result<Vec<Uav>> {
            let mut stmt = conn
                .prepare(
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
                )
                .unwrap();

            let mut uav_vector: Vec<Uav> = Vec::new();

            let uav_iter = stmt
                .query_map([], |row| {
                    Ok(Uav {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        max_payload_mass: row.get(2)?,
                        flight_duration: row.get(3)?,
                        takeoff_speed: row.get(4)?,
                        flight_speed: row.get(5)?,
                        minimum_altitude: row.get(6)?,
                        max_altitude: row.get(7)?,
                    })
                })
                .unwrap();

            for uav_item in uav_iter {
                uav_vector.push(uav_item.unwrap());
            }
            Ok(uav_vector)
        }
    }
}
