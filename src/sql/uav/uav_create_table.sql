CREATE TABLE "uav" (
    -- идентификатор
    "uav_id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- наименование дрона
    "uav_name" TEXT NOT NULL,
    -- максимальная маса полезной нагрузки г.
    "uav_max_payload_mass" INTEGER NOT NULL CHECK (uav_max_payload_mass >= 0),
    -- продолжительность полёта с.
    "uav_flight_duration" INTEGER NOT NULL CHECK (uav_flight_duration >= 0),
    -- скорость взлёта м/c
    "uav_takeoff_speed" DECIMAL(4, 3) NOT NULL CHECK (uav_takeoff_speed >= 0),
    -- скорость полёта м/c
    "uav_flight_speed" DECIMAL(4, 3) NOT NULL CHECK (uav_flight_speed >= 0),
    -- минимальная высота полёта м.
    "uav_min_altitude" DECIMAL(4, 3) DEFAULT 0 NOT NULL CHECK (uav_minimum_altitude >= 0),
    -- максимальная высота полёта м.
    "uav_max_altitude" DECIMAL(4, 3) DEFAULT 0 NOT NULL CHECK (uav_max_altitude >= 0),
)