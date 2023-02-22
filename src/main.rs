mod camera;
mod uav;

use camera::camera_mod::Camera;
use rusqlite::{Connection, Result};
use uav::uav_mod::Uav;

fn main() -> Result<()> {
    // let conn = Connection::open_in_memory()?;
    let conn = Connection::open("mydatabase.db")?;

    Uav::sql_create_table(&conn);

    Camera::sql_create_table(&conn);

    // match Camera::sql_create_table(&conn)
    // {
    //     Ok(_) => {println!("Table created")},
    //     Err(e) => {println!("{}", e)},
    // };

    // let mut my_uav = Uav::new(String::from("Fake Drone 2"), 1000, 6000, 10.0, 50.0, 20.000000001, 500.0);

    // match my_uav.sql_add_to_db(&conn)
    // {
    //     Ok(_) => {println!("Added to db")},
    //     Err(e) => {println!("{}", e)},
    // };

    // my_uav.name = "Drone 2".to_string();

    //my_uav.sql_add_to_db(&conn).unwrap();
    // {
    //     let uav_iter = Uav::sql_get_uavs(&conn).unwrap();

    //     for mut uav_item in uav_iter {
    //         uav_item.max_payload_mass = 241;
    //         uav_item.sql_update(&conn).unwrap();
    //     }
    // }

    let uav_iter = Uav::sql_get_uavs(&conn).unwrap();

    for uav_item in uav_iter {
        match uav_item {
            Ok(uav) => uav.print_uav(),
            Err(_) => println!("Error =("),
        };
    }

    // let mut my_camera = Camera::new(
    //     String::from("Camera 12"),
    //     10003,
    //     150,
    //     360,
    // );

    // match my_camera.sql_add_to_db(&conn)
    // {
    //     Ok(_) => {println!("Added to db")},
    //     Err(e) => {println!("{}", e)},
    // };

    // my_camera.name = "Camera 2".to_string();
    // my_camera.sql_add_to_db(&conn).unwrap();

    let camera_iter = Camera::sql_get_cameras(&conn).unwrap();

    for camera_item in camera_iter {
        match camera_item
        {
            Ok(camera) => camera.print_camera(),
            Err(_) => println!("err =("),
        }
    }

    // let uav_iter = Uav::sql_get_uavs(&conn).unwrap();

    // for uav_item in uav_iter {
    //     println!("{:?}", uav_item);
    // }

    Ok(())
}
