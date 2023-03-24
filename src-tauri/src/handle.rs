use rusqlite::{Connection, Result};

use crate::mission;
use mission::{camera::Camera, uav::Uav};

#[tauri::command]
pub fn save_uav(conn: &Connection, uav: mission::uav::Uav) -> Result<()> {
    println!("Received updated UAV: {:?}", uav);
    // Update the backend with the new UAV data
    Ok(())
}

#[tauri::command]
pub fn update_uav(uav: Uav) -> String {
    let conn = Connection::open("mydatabase.db").expect("Cant open base");
    println!("Received updated UAV: {:?}", uav);
    match uav.sql_update(&conn) {
        Ok(_) => "Ok".to_string(),
        Err(e) => e.to_string(),
    }
}
