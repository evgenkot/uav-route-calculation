#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde::Deserialize;

#[derive(Deserialize)]
struct PolygonCoordinates {
    vertices: Vec<Vec<[f64; 2]>>,
}


#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}


#[tauri::command]
fn receive_polygon_coordinates(vertices: Vec<Vec<[f64; 2]>>) {
    println!("Received polygon coordinates: {:?}", vertices);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, receive_polygon_coordinates])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
