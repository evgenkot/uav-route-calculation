CREATE TABLE "camera" (
    -- идентификатор
    "camera_id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- наименование камеры
    "camera_name" TEXT NOT NULL,
    -- масса камеры г.
    "camera_mass" DECIMAL(10, 3),
    -- угол обзора камеры по координате x градусы
    "camera_fov_x" INTEGER NOT NULL CHECK (
        camera_fov_x >= 0
        AND camera_fov_x <= 360
    ),
    -- угол обзора камеры по координате y градусы
    "camera_fov_y" INTEGER NOT NULL CHECK (
        camera_fov_y >= 0
        AND camera_fov_y <= 360
    ),
);