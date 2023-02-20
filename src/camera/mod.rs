pub mod camera_mod {
    use rusqlite::{Connection, Result};

    #[derive(Debug)]
    pub struct Camera {
        id: u64,
        pub name: String,
        pub mass: u64,
        pub fov_x: u16,
        pub fov_y: u16,
    }

    impl Camera {
        pub fn new(
            name: String,
            mass: u64,
            fov_x: u16,
            fov_y: u16,
        ) -> Camera {
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

        pub fn sql_create_table(conn: &Connection) {
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
            .unwrap();
        }

        pub fn sql_add_to_db(&self, conn: &Connection) -> Result<()> {
            conn.execute(
                "INSERT INTO camera (
                    camera_name,
                    camera_mass,
                    camera_fov_x,
                    camera_fov_y
                ) VALUES (?1, ?2, ?3, ?4)",
                (
                    &self.name,
                    &self.mass,
                    &self.fov_x,
                    &self.fov_y,
                ),
            )
            .unwrap();
            Ok(())
        }

        pub fn sql_update(&self, conn: &Connection) -> Result<()> {
            conn.execute(
                "
                    UPDATE camera SET
                        camera_name = ?1,
                        camera_mass = ?2,
                        camera_fov_x = ?3,
                        camera_fov_y = ?4
                    WHERE uav_id = ?5",
                (
                    &self.name,
                    &self.mass,
                    &self.fov_x,
                    &self.fov_y,
                    &self.id,
                ),
            )
            .unwrap();
            Ok(())
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
                )
                .unwrap();

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
