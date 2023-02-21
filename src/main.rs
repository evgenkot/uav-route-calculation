mod camera;
mod uav;

use camera::camera_mod::Camera;
use rusqlite::{Connection, Result};
use uav::uav_mod::Uav;

fn main() -> Result<()> {
    // let conn = Connection::open_in_memory()?;
    let conn = Connection::open("mydatabase.db")?;


    // UAV
    match Uav::sql_create_table(&conn)
    {
        Ok(_) => {println!("Table uav created")},
        Err(e) => {println!("Table uav not created {}", e)}
    };

    let my_uav = Uav::new(
        String::from("Fake Drone 1"), 
        1000, 
        6000, 
        10.0, 
        50.0, 
        20.000000001, 
        500.0);

    match my_uav.sql_add_to_db(&conn)
    {
        Ok(_) => {println!("Added to db")},
        Err(e) => {println!("{}", e)}
    };

    let uav_iter = Uav::sql_get_uavs(&conn).unwrap();


    for uav_item in uav_iter {
        println!("{:?}", uav_item);
    }

    // Camera
    match Camera::sql_create_table(&conn)
    {
        Ok(_) => {println!("Table camera created")},
        Err(e) => {println!("Table camera not created {}", e)}
    };

    let my_camera = Camera::new(
        String::from("Camera 132"),
        10003,
        150,
        360,
    );

    match my_camera.sql_add_to_db(&conn)
    {
        Ok(_) => {println!("Added to db")},
        Err(e) => {println!("{}", e)}
    };

    let camera_iter = Camera::sql_get_cameras(&conn).unwrap();


    for camera_item in camera_iter {
        println!("{:?}", camera_item);
    }

    // match my_uav.sql_add_to_db(&conn)
    // {
    //     Ok(_) => {print!()}
    // };



    

    // let mut my_uav = Uav::new(String::from("Fake Drone 1"), 1000, 6000, 10.0, 50.0, 20.000000001, 500.0);

    

    // my_uav.name = "Drone 2".to_string();

    //my_uav.sql_add_to_db(&conn).unwrap();
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

    

    // my_camera.name = "Camera 2".to_string();
    // my_camera.sql_add_to_db(&conn).unwrap();

    

    // let uav_iter = Uav::sql_get_uavs(&conn).unwrap();

    // for uav_item in uav_iter {
    //     println!("{:?}", uav_item);
    // }

    Ok(())
}
