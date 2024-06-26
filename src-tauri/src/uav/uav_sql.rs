use crate::uav::Uav;
use rusqlite::{Connection, Result};

pub fn create_table(conn: &Connection) -> Result<usize> {
    let db_create = conn.execute(
        "CREATE TABLE IF NOT EXISTS uav (
                uav_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                uav_name TEXT NOT NULL,
                uav_max_payload_mass INTEGER NOT NULL CHECK (uav_max_payload_mass >= 0),
                uav_flight_duration INTEGER NOT NULL CHECK (uav_flight_duration >= 0),
                uav_takeoff_speed REAL NOT NULL CHECK (uav_takeoff_speed >= 0),
                uav_flight_speed REAL NOT NULL CHECK (uav_flight_speed >= 0),
                uav_min_altitude REAL DEFAULT 0 NOT NULL CHECK (uav_min_altitude >= 0),
                uav_max_altitude REAL DEFAULT 0 NOT NULL CHECK (uav_max_altitude >= 0),
                camera_id INTEGER,
                    FOREIGN KEY (camera_id) 
                    REFERENCES camera (camera_id) 
                    ON DELETE SET NULL
                )",
        (),
    );

    let index_create = conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS camera_id_index
        ON uav(camera_id)",
        (),
    );

    match (db_create, index_create) {
        (Ok(val1), Ok(val2)) => Ok(val1 + val2),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

pub fn insert(uav: &Uav, conn: &Connection) -> Result<usize> {
    conn.execute(
        "INSERT INTO uav (
                uav_name,
                uav_max_payload_mass,
                uav_flight_duration,
                uav_takeoff_speed,
                uav_flight_speed,
                uav_min_altitude,
                uav_max_altitude,
                camera_id
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        (
            &uav.name,
            &uav.max_payload_mass,
            &uav.flight_duration,
            &uav.takeoff_speed,
            &uav.flight_speed,
            &uav.min_altitude,
            &uav.max_altitude,
            &uav.camera_id,
        ),
    )
}

pub fn update(uav: &Uav, conn: &Connection) -> Result<usize> {
    conn.execute(
        "
                UPDATE uav SET
                    uav_name = ?1,
                    uav_max_payload_mass = ?2,
                    uav_flight_duration = ?3,
                    uav_takeoff_speed = ?4,
                    uav_flight_speed = ?5,
                    uav_min_altitude = ?6,
                    uav_max_altitude = ?7,
                    camera_id = ?8
                WHERE uav_id = ?9",
        (
            &uav.name,
            &uav.max_payload_mass,
            &uav.flight_duration,
            &uav.takeoff_speed,
            &uav.flight_speed,
            &uav.min_altitude,
            &uav.max_altitude,
            &uav.camera_id,
            &uav.id,
        ),
    )
}

pub fn delete(uav: &Uav, conn: &Connection) -> Result<usize> {
    conn.execute(
        "DELETE FROM uav 
        WHERE uav_id = ?1",
        (&uav.id,),
    )
}

pub fn get_uavs(conn: &Connection) -> Result<Vec<Result<Uav>>> {
    let mut stmt = conn.prepare(
        "SELECT
                    uav_id,
                    uav_name,
                    uav_max_payload_mass,
                    uav_flight_duration,
                    uav_takeoff_speed,
                    uav_flight_speed,
                    uav_min_altitude,
                    uav_max_altitude,
                    camera_id
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
            camera_id: row.get(8)?,
        })
    })?;

    Ok(uav_iter.collect())
}

pub fn get_uavs_vec(conn: &Connection) -> Result<Vec<Uav>> {
    let uav_results = get_uavs(conn)?;

    let mut uavs: Vec<Uav> = Vec::new();
    for uav_result in uav_results {
        match uav_result {
            Ok(uav) => uavs.push(uav),
            Err(err) => eprintln!("Error processing a UAV: {}", err),
        }
    }
    Ok(uavs)
}
