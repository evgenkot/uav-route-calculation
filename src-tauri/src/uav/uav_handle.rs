use crate::uav::uav_sql;
use crate::uav::Uav;
use rusqlite::Connection;

#[tauri::command]
pub fn new_uav(uav: Uav) -> String {
    let conn = Connection::open("mydatabase.db").expect("Cant open base");
    println!("Received new UAV: {:?}", uav);
    match uav_sql::insert(&uav, &conn) {
        Ok(_) => "Ok".to_string(),
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
pub fn update_uav(uav: Uav) -> String {
    let conn = Connection::open("mydatabase.db").expect("Cant open base");
    println!("Received updated UAV: {:?}", uav);
    match uav_sql::update(&uav, &conn) {
        Ok(_) => "Ok".to_string(),
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
pub fn delete_uav(uav: Uav) -> String {
    let conn = Connection::open("mydatabase.db").expect("Cant open base");
    println!("Received delete UAV: {:?}", uav);
    match uav_sql::delete(&uav, &conn) {
        Ok(_) => "Ok".to_string(),
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
pub fn get_all_uavs_vec() -> Vec<Uav> {
    let conn = Connection::open("mydatabase.db").expect("Cant open base");
    match uav_sql::get_uavs_vec(&conn) {
        Ok(result) => result,
        Err(_) => vec![],
    }
}
