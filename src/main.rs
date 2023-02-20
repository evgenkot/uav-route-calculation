mod uav;
mod camera;

use rusqlite::{Connection, Result};
use uav::uav_mod::Uav;
use camera::camera_mod::Camera;

fn main() -> Result<()> {
    // let conn = Connection::open_in_memory()?;
    let conn = Connection::open("mydatabase.db")?;

    Uav::sql_create_table(&conn);
    Camera::sql_create_table(&conn);

    // let mut my_uav = Uav::new(String::from("Drone 1"), 1000, 6000, 10.0, 50.0, 10.0, 500.0);

    // my_uav.sql_add_to_db(&conn).unwrap();

    // my_uav.name = "Drone 2".to_string();

    // my_uav.sql_add_to_db(&conn).unwrap();
    // {
    //     let uav_iter = Uav::sql_get_uavs(&conn).unwrap();

    //     for mut uav_item in uav_iter {
    //         uav_item.max_payload_mass = 241;
    //         uav_item.sql_update(&conn).unwrap();
    //     }
    // }

    // let uav_iter = Uav::sql_get_uavs(&conn).unwrap();

    // for uav_item in uav_iter {
    //     println!("{:?}", uav_item);
    // }

    
    
    // let mut my_camera = Camera::new(
    //     String::from("Camera 1"),
    //     1000,
    //     150,
    //     80,
    // );

    // my_camera.sql_add_to_db(&conn).unwrap();
    
    // my_camera.name = "Camera 2".to_string();
    // my_camera.sql_add_to_db(&conn).unwrap();

    let camera_iter = Camera::sql_get_cameras(&conn).unwrap();

    for camera_item in camera_iter {
        println!("{:?}", camera_item);
    }

    let uav_iter = Uav::sql_get_uavs(&conn).unwrap();

    for uav_item in uav_iter {
        println!("{:?}", uav_item);
    }





    Ok(())
}
