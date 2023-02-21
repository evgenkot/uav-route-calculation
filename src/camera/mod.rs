pub mod camera_mod {
    use rusqlite::{Connection, Result};

    #[derive(Debug)]
    pub struct Camera {
        id: u64,          // id
        pub name: String, // name
        pub mass: u64,    // mass in grams
        pub fov_x: u16,   // x-axis viewing angle in degrees
        pub fov_y: u16,   // y-axis viewing angle in degrees
    }

    impl Camera {
        pub fn new(name: String, mass: u64, fov_x: u16, fov_y: u16) -> Camera {
            Camera {
                id: 0,
                name,
                mass,
                fov_x,
                fov_y,
            }
        }

        pub fn take_picture(&self) {
            println!("{} Click!", self.name);
        }

        pub fn sql_create_table(conn: &Connection) -> Result<usize> {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS camera (
                    camera_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                    camera_name TEXT NOT NULL,
                    camera_mass INTEGER NOT NULL CHECK (camera_mass >= 0),
                    camera_fov_x INTEGER NOT NULL CHECK (
                        camera_fov_x >= 0
                        AND camera_fov_x <= 360
                    ),
                    camera_fov_y INTEGER NOT NULL CHECK (
                        camera_fov_y >= 0
                        AND camera_fov_y <= 360
                    )
                    )",
                (),
            )
        }

        pub fn sql_add_to_db(&self, conn: &Connection) -> Result<usize> {
            conn.execute(
                "INSERT INTO camera (
                    camera_name,
                    camera_mass,
                    camera_fov_x,
                    camera_fov_y
                ) VALUES (?1, ?2, ?3, ?4)",
                (&self.name, &self.mass, &self.fov_x, &self.fov_y),
            )
        }

        pub fn sql_update(&self, conn: &Connection) -> Result<usize> {
            conn.execute(
                "
                    UPDATE camera SET
                        camera_name = ?1,
                        camera_mass = ?2,
                        camera_fov_x = ?3,
                        camera_fov_y = ?4
                    WHERE uav_id = ?5",
                (&self.name, &self.mass, &self.fov_x, &self.fov_y, &self.id),
            )
        }

        pub fn sql_get_cameras(conn: &Connection) -> Result<Vec<Camera>> {
            let mut stmt = conn
                .prepare(
                    "SELECT
                    camera_id,
                    camera_name,
                    camera_mass,
                    camera_fov_x,
                    camera_fov_y
                    FROM camera",
                ).unwrap();

            let mut camera_vector: Vec<Camera> = Vec::new();

            let camera_iter = stmt
                .query_map([], |row| {
                    Ok(Camera {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        mass: row.get(2)?,
                        fov_x: row.get(3)?,
                        fov_y: row.get(4)?,
                    })
                })
                .unwrap();

            for camera_item in camera_iter {
                camera_vector.push(camera_item.unwrap());
            }
            Ok(camera_vector)
        }
    }
}
