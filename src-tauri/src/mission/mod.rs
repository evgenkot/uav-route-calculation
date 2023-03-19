use serde::Deserialize;
pub(crate) mod uav;
pub(crate) mod camera;

#[derive(Deserialize)]
pub struct PolygonCoordinates {
    vertices: Vec<Vec<[f64; 2]>>,
}

// #[tauri::command]
// pub fn receive_polygon_coordinates(coordinates: PolygonCoordinates) {
//     println!("Received polygon coordinates: {:?}", coordinates.vertices);
// }
#[tauri::command]
pub fn receive_polygon_coordinates(vertices: Vec<Vec<[f64; 2]>>) {
    println!("Received polygon coordinates: {:?}", vertices);
}


pub enum DroneAction {
    TakeOff,
    TakePhoto,
    Recharge,
}

pub struct Waypoint {
    pub coordinates: [f64; 2], // waypoint coordinates (latitude and longitude)
    pub action: DroneAction, // action to perform at this waypoint
}

pub struct Mission {
    pub id: u64, // mission id
    pub name: String, // mission name
    pub uav: uav::Uav, // UAV to be used in the mission
    pub camera: camera::Camera, // Camera to be used in the mission
    pub flight_altitude: f64, // flight altitude in meters
    pub photo_angle: f64, // angle of photo direction in degrees
    pub coordinates: Vec<Vec<[f64; 2]>>, // polygon coordinates for the mission area
    pub results: Option<MissionResults>, // results of the mission (optional, to be calculated)
}

pub struct MissionResults {
    pub total_flight_distance: f64, // total flight distance in meters
    pub photos_taken: u64, // number of photos taken during the mission
    pub continuation_point: Option<[f64; 2]>, // point at which the drone will continue its work
    pub operator_relocation_point: Option<[f64; 2]>, // point where the operator should move
    pub waypoints: Vec<Waypoint>, // sequence of waypoints for the drone to visit
}
