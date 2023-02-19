CREATE TABLE uav (
    uav_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    uav_name TEXT NOT NULL,
    uav_max_payload_mass INTEGER NOT NULL CHECK (uav_max_payload_mass >= 0),
    uav_flight_duration INTEGER NOT NULL CHECK (uav_flight_duration >= 0),
    uav_takeoff_speed DECIMAL(4, 3) NOT NULL CHECK (uav_takeoff_speed >= 0),
    uav_flight_speed DECIMAL(4, 3) NOT NULL CHECK (uav_flight_speed >= 0),
    uav_min_altitude DECIMAL(4, 3) DEFAULT 0 NOT NULL CHECK (uav_min_altitude >= 0),
    uav_max_altitude DECIMAL(4, 3) DEFAULT 0 NOT NULL CHECK (uav_max_altitude >= 0)
);