#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rusqlite::{Connection, Result};
mod camera_handle;
mod mission;
mod uav_handle;
use mission::{camera::Camera, uav::Uav};

use crate::mission::uav;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    {
        let conn = Connection::open("mydatabase.db").expect("Cant open base");

        mission::uav::Uav::sql_create_table(&conn).expect("cant create uav table");
        mission::camera::Camera::sql_create_table(&conn).expect("Cant create camera table");

        

        for i in 1..10 {
            let mut ccamera = Camera::new_random();
            ccamera.sql_add_to_db( &conn).expect("Allo");
        }
        for i in 1..10 {
            let mut ccamera = Uav::new_random();
            ccamera.sql_add_to_db( &conn).expect("Allo");
        }

        match Uav::get_uavs(&conn) {
            Ok(uavs) => {


                for uav in uavs {
                    println!("UAV: {:?}", uav);
                }
            }
            Err(err) => {
                eprintln!("Error fetching UAVs: {}", err);
            }
        }

        match Camera::get_cameras(&conn) {
            Ok(cameras) => {


                for camera in cameras {
                    println!("Camera: {:?}", camera);
                }
            }
            Err(err) => {
                eprintln!("Error fetching UAVs: {}", err);
            }
        }
        
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            mission::receive_polygon_coordinates,
            uav_handle::new_uav,
            uav_handle::update_uav,
            uav_handle::delete_uav,
            uav_handle::get_uavs_vec,
            camera_handle::new_camera,
            camera_handle::update_camera,
            camera_handle::delete_camera,
            camera_handle::get_cameras_vec,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
