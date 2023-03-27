#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rusqlite::{Connection, Result};
mod mission;
mod uav_handle;
use mission::{camera::Camera, uav::Uav};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    {
    let conn = Connection::open("mydatabase.db").expect("Cant open base");

    mission::uav::Uav::sql_create_table(&conn).expect("cant create uav table");
    mission::camera::Camera::sql_create_table(&conn).expect("Cant create camera table");
    }

    // match Uav::get_uavs(&conn) {
    //     Ok(uavs) => {
    //         for uav in uavs {
    //             println!("UAV: {:?}", uav);
    //         }
    //     }
    //     Err(err) => {
    //         eprintln!("Error fetching UAVs: {}", err);
    //     }
    // }


    // let camera_iter = Camera::sql_get_cameras(&conn).unwrap();

    // for camera_item in camera_iter {
    //     match camera_item {
    //         Ok(camera) => camera.print_camera(),
    //         Err(_) => println!("err =("),
    //     }
    // }



    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            mission::receive_polygon_coordinates,
            uav_handle::new_uav,
            uav_handle::update_uav,
            uav_handle::delete_uav,
            uav_handle::get_uavs_vec,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
