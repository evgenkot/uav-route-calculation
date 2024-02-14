#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rusqlite::Connection;
mod algorithms;

mod camera;
mod uav;

use uav::uav_handle;

use camera::camera_handle;

fn main() {
    {
        let conn = Connection::open("mydatabase.db").expect("Cant open base");

        uav::uav_sql::create_table(&conn).expect("cant create uav table");
        camera::camera_sql::create_table(&conn).expect("Cant create camera table");
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            uav_handle::new_uav,
            uav_handle::update_uav,
            uav_handle::delete_uav,
            uav_handle::get_all_uavs_vec,
            camera_handle::new_camera,
            camera_handle::update_camera,
            camera_handle::delete_camera,
            camera_handle::get_all_cameras_vec,
            algorithms::discretize_area,
            algorithms::nearest_neighbor,
            algorithms::brute_force,
            algorithms::rectangular_areas,
            algorithms::calculate_distance,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
