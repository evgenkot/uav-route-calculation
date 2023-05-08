use crate::camera::Camera;
use rusqlite::Connection;

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
pub fn get_cameras_vec() -> Vec<Camera> {
    let conn = Connection::open("mydatabase.db").expect("Cant open base");
    match Camera::get_cameras(&conn) {
        Ok(result) => result,
        Err(_) => vec![],
    }
}
