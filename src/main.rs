mod uav;

use rusqlite::{Connection, Result};
use uav::uav_mod::Uav;

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    Uav::sql_create_table(&conn);

    let mut my_uav = Uav::new(String::from("Drone 1"), 1000, 6000, 10.0, 50.0, 10.0, 500.0);

    my_uav.sql_add_to_db(&conn).unwrap();

    my_uav.name = "Drone 2".to_string();
    my_uav.sql_add_to_db(&conn).unwrap();

    let uav_iter = Uav::sql_get_uavs(&conn).unwrap();

    for uav_item in uav_iter {
        println!("{:?}", uav_item);
    }

    Ok(())
}
