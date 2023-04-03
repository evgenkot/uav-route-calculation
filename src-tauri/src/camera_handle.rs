use rusqlite::{Connection, Result};

use crate::mission;
use mission::{camera::Camera};

// #[tauri::command]
// pub fn save_uav(conn: &Connection, uav: mission::uav::Uav) -> Result<()> {
//     println!("Received updated UAV: {:?}", uav);
//     // Update the backend with the new UAV data
//     Ok(())
// }

#[tauri::command]
pub fn new_camera(camera: Camera) -> String {
    let conn = Connection::open("mydatabase.db").expect("Cant open base");
    println!("Received new camera: {:?}", camera);
    match camera.sql_add_to_db(&conn) {
        Ok(_) => "Ok".to_string(),
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
pub fn update_camera(camera: Camera) -> String {
    let conn = Connection::open("mydatabase.db").expect("Cant open base");
    println!("Received updated camera: {:?}", camera);
    match camera.sql_update(&conn) {
        Ok(_) => "Ok".to_string(),
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
pub fn delete_camera(camera: Camera) -> String {
    let conn = Connection::open("mydatabase.db").expect("Cant open base");
    println!("Received delete camera: {:?}", camera);
    match camera.sql_delete(&conn) {
        Ok(_) => "Ok".to_string(),
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
pub fn get_cameras_vec() -> Vec<mission::camera::Camera> {
    let conn = Connection::open("mydatabase.db").expect("Cant open base");
    match mission::camera::Camera::get_cameras(&conn) {
        Ok(result) => result,
        Err(_) => vec![],
    }
}
