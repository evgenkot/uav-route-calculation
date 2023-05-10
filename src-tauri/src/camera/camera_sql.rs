use crate::camera::Camera;
use rusqlite::{Connection, Result};

pub fn create_table(conn: &Connection) -> Result<usize> {
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

pub fn insert(camera: &Camera, conn: &Connection) -> Result<usize> {
    conn.execute(
        "INSERT INTO camera (
                camera_name,
                camera_mass,
                camera_fov_x,
                camera_resolution_x,
                camera_resolution_y
            ) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            &camera.name,
            &camera.mass,
            &camera.fov_x,
            &camera.resolution_x,
            &camera.resolution_y,
        ),
    )
}

pub fn update(camera: &Camera, conn: &Connection) -> Result<usize> {
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
            &camera.name,
            &camera.mass,
            &camera.fov_x,
            &camera.resolution_x,
            &camera.resolution_y,
            &camera.id,
        ),
    )
}

pub fn delete(camera: &Camera, conn: &Connection) -> Result<usize> {
    conn.execute(
        "DELETE FROM camera 
        WHERE camera_id = ?1",
        (&camera.id,),
    )
}

pub fn get_cameras(conn: &Connection) -> Result<Vec<Result<Camera>>> {
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

    Ok(camera_iter.collect())
}
pub fn get_cameras_vec(conn: &Connection) -> Result<Vec<Camera>> {
    let camera_results = get_cameras(conn)?;

    let mut cameras: Vec<Camera> = Vec::new();
    for camera_result in camera_results {
        match camera_result {
            Ok(camera) => cameras.push(camera),
            Err(err) => eprintln!("Error processing a Camera: {}", err),
        }
    }
    Ok(cameras)
}
