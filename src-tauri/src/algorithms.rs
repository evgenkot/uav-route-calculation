use std::f64::consts::PI;
use std::f64::{INFINITY, NEG_INFINITY};

// Coordinate transformation at rotation, they express old coordinates through new coordinates
fn coordinate_restore(x: f64, y: f64, direction_radians: f64) -> (f64, f64) {
    let cosinus = direction_radians.cos();
    let sinus = direction_radians.sin();
    let xd = x * cosinus - y * sinus;
    let yd = x * sinus + y * cosinus;
    (xd, yd)
}

fn x_restore(x: f64, y: f64, direction_radians: f64) -> f64 {
    x * direction_radians.cos() - y * direction_radians.sin()
}

fn y_restore(x: f64, y: f64, direction_radians: f64) -> f64 {
    x * direction_radians.sin() + y * direction_radians.cos()
}

// Coordinate transformation at rotation, they express old coordinates through new coordinates
fn coordinate_transformation(x: f64, y: f64, direction_radians: f64) -> (f64, f64) {
    let cosinus = direction_radians.cos();
    let sinus = direction_radians.sin();
    let xd = x * cosinus + y * sinus;
    let yd = y * cosinus - x * sinus;
    (xd, yd)
}

fn x_transform(x: f64, y: f64, direction_radians: f64) -> f64 {
    x * direction_radians.cos() + y * direction_radians.sin()
}

fn y_transform(x: f64, y: f64, direction_radians: f64) -> f64 {
    y * direction_radians.cos() - x * direction_radians.sin()
}

#[tauri::command]
pub fn discretize_area(
    // Vector of tuples representing x and y coordinates of the polygon.
    polygons: Vec<Vec<(f64, f64)>>,
    // Width of the photo.
    photo_width: f64,
    // Height of the photo.
    photo_height: f64,
    // Direction
    direction_degrees: f64,
    // Verification whether the points are inside the polygon
    check_inside: bool,
) -> Result<Vec<Vec<Vec<(f64, f64)>>>, String> {
    // Returns a Result containing either a vector of tuples representing the discretized area or a String error.
    println!("Received polygon coordinates: {:?}", polygons);

    let direction_radians = direction_degrees * PI / 180.0;

    // Initialize a vector to store the results for each polygon.
    let mut results = Vec::new();

    for polygon in polygons {
        // Initialize min and max x and y values to extreme opposites.
        let (mut min_x, mut max_x, mut min_y, mut max_y) =
            (INFINITY, NEG_INFINITY, INFINITY, NEG_INFINITY);

        // Apply transformation to each point
        let polygon_transformed: Vec<(f64, f64)> = polygon
            .iter()
            .map(|&(x, y)| coordinate_transformation(x, y, direction_radians))
            .collect();

        // Loop through polygon coordinates to find min and max x and y values.
        for (x, y) in &polygon_transformed {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }

        // Inner function to check if a point is inside the polygon.
        fn is_inside_polygon(point: (f64, f64), polygon: &Vec<(f64, f64)>) -> bool {
            // Initialize inside flag to false.
            let mut inside = false;
            let len = polygon.len();
            let mut j = len - 1;

            // Loop through the polygon's vertices, checking if the point intersects.
            for i in 0..len {
                let (x_i, y_i) = polygon[i];
                let (x_j, y_j) = polygon[j];

                // Determine if the point is intersecting with the edge from vertex i to vertex j.
                let intersect = (y_i > point.1) != (y_j > point.1)
                    && point.0 < (x_j - x_i) * (point.1 - y_i) / (y_j - y_i) + x_i;
                if intersect {
                    inside = !inside;
                }
                j = i;
            }
            inside
        }

        // Initialize the result vector.
        let mut result = Vec::new();

        // Calculate half the camera width and height.
        let (half_camera_width, half_camera_height) = (photo_width / 2.0, photo_height / 2.0);

        let polygon_width = (max_x - min_x).abs();
        let polygon_height = (max_y - min_y).abs();

        let photo_count_width = (polygon_width / photo_width) as u64 + 1;
        let photo_count_height = (polygon_height / photo_height) as u64 + 1;

        for i in 0..photo_count_width {
            let x = min_x + (i as f64) * photo_width;
            let mut line = Vec::new();
            for j in 0..photo_count_height {
                let y = min_y + (j as f64) * photo_height;

                if check_inside {
                    // Calculate the corners of the rectangle at (x, y).
                    let corners = vec![
                        (x, y),
                        (x + photo_width, y),
                        (x, y + photo_height),
                        (x + photo_width, y + photo_height),
                    ];

                    // Check if any corner of the rectangle is inside the polygon.
                    let is_any_corner_inside = corners
                        .iter()
                        .any(|&corner| is_inside_polygon(corner, &polygon_transformed));

                    // If any corner is inside, calculate the center of the rectangle and add it to the result.
                    if is_any_corner_inside {
                        let center_x = x + half_camera_width;
                        let center_y = y + half_camera_height;
                        line.push(coordinate_restore(center_x, center_y, direction_radians));
                    }
                } else {
                    let center_x = x + half_camera_width;
                    let center_y = y + half_camera_height;
                    line.push(coordinate_restore(center_x, center_y, direction_radians));
                }
            }
            result.push(line);
        }
        // Push the result vector containing the centers of the rectangles that intersect with the polygon.
        results.push(result);
    }
    // Return the vector of results containing the centers of the rectangles that intersect with each polygon.
    Ok(results)
}

#[tauri::command]
pub fn nearest_neighbor(
    points: Vec<(f64, f64)>,
    start_point: (f64, f64),
) -> Result<Vec<(f64, f64)>, String> {
    if points.is_empty() {
        return Err("The input points must not be empty".to_string());
    }

    let mut remaining_points: Vec<(f64, f64)> = points;
    let mut result: Vec<(f64, f64)> = Vec::new();
    let mut current_point = start_point;

    while !remaining_points.is_empty() {
        let (nearest_index, nearest_point) = remaining_points
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                euclidean_distance(&current_point, a)
                    .partial_cmp(&euclidean_distance(&current_point, b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .ok_or("Failed to find the nearest point".to_string())?;
        let nearest_point = *nearest_point;
        remaining_points.remove(nearest_index);
        if result.is_empty() {
            result.push(start_point);
        }
        result.push(nearest_point);
        current_point = nearest_point;
    }

    result.push(start_point);
    Ok(result)
}
// Helper function to calculate the Euclidean distance between two points
pub fn euclidean_distance(a: &(f64, f64), b: &(f64, f64)) -> f64 {
    let (x1, y1) = *a;
    let (x2, y2) = *b;
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

// pub fn christofides_algorithm(points: &[(f64, f64)], start_point: &(f64, f64)) -> Vec<(f64, f64)> {
//     // Implement the christofides_algorithm
// }

use std::sync::{Arc, Mutex};
use std::thread;

// Main function to find the shortest path using the brute force approach.
#[tauri::command]
pub fn brute_force(points: Vec<(f64, f64)>, start_point: (f64, f64)) -> Vec<(f64, f64)> {
    // Wrap the points and best_path in Arc for shared ownership across threads.
    let points = Arc::new(points);
    let best_path = Arc::new(Mutex::new((Vec::new(), f64::MAX)));

    let mut threads = Vec::new();

    // Iterate through each point, creating a separate thread for each.
    for (i, point) in points.iter().enumerate() {
        // Clone Arc references to share ownership with the spawned threads.
        let points = Arc::clone(&points);
        let best_path = Arc::clone(&best_path);
        let start_point = start_point.clone();
        let removed_point = point.clone();

        // Spawn a thread for each point, removing it from the remaining points.
        let thread = thread::spawn(move || {
            let mut remaining_points = points.to_vec();
            remaining_points.remove(i);

            let current_path = vec![start_point, removed_point];
            let current_distance = euclidean_distance(&start_point, &removed_point);

            // Call the helper function in each thread.
            brute_force_helper(
                remaining_points,
                current_path,
                start_point,
                current_distance,
                &best_path,
            );
        });

        threads.push(thread);
    }

    // Wait for all threads to complete.
    for thread in threads {
        thread.join().unwrap();
    }

    // Extract the best path from the Arc<Mutex<_>>.
    let (best_path, _) = Arc::try_unwrap(best_path).unwrap().into_inner().unwrap();
    best_path
}

// Recursive helper function to find the shortest path using the brute force approach.
fn brute_force_helper(
    points: Vec<(f64, f64)>,
    current_path: Vec<(f64, f64)>,
    start_point: (f64, f64),
    current_distance: f64,
    best_path: &Arc<Mutex<(Vec<(f64, f64)>, f64)>>,
) {
    // Base case: if there are no points left, update the best path if the current path is shorter.
    if points.is_empty() {
        let total_distance =
            current_distance + euclidean_distance(&start_point, current_path.last().unwrap());

        let mut best_path = best_path.lock().unwrap();

        if total_distance < best_path.1 {
            best_path.0 = current_path.clone();
            best_path.0.push(start_point);
            best_path.1 = total_distance;
        }
    } else {
        // Iterate through each remaining point, removing it from the list and updating the path.
        for (i, point) in points.iter().enumerate() {
            let mut remaining_points = points.clone();
            let removed_point = remaining_points.remove(i);

            let last_point = current_path.last().unwrap();
            let new_distance = current_distance + euclidean_distance(last_point, &removed_point);

            // If the updated path is shorter than the current best path, continue the search.
            if new_distance < best_path.lock().unwrap().1 {
                let mut new_path = current_path.clone();
                new_path.push(*point);

                brute_force_helper(
                    remaining_points,
                    new_path,
                    start_point,
                    new_distance,
                    best_path,
                );
            }
        }
    }
}

#[tauri::command]
pub fn calculate_distance(points: Vec<(f64, f64)>) -> f64 {
    points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .map(|(a, b)| euclidean_distance(a, b))
        .sum()
}

#[tauri::command]
pub fn rectangular_areas(
    points: Vec<Vec<Vec<(f64, f64)>>>,
    start_point: (f64, f64),
    // Direction for quick calculation of polygon distance
    direction_degrees: f64,
) -> Result<Vec<(f64, f64)>, String> {
    if points.is_empty() {
        return Err("The input points must not be empty".to_string());
    }
    let direction_radians = direction_degrees * PI / 180.0;
    let mut calculation_result: Vec<(f64, f64)> = Vec::new();
    let mut multiple_region_result: Vec<Vec<(f64, f64)>> = Vec::new();

    let region_count = points.len();
    let mut weights: Vec<Vec<Option<(f64, Direction)>>> =
        vec![vec![None; region_count]; region_count];

    for i in 0..region_count - 1 {
        let i_height = points[i][0].len();
        let i_width = points[i].len();
        for j in i + 1..region_count {
            let j_height = points[j][0].len();
            let j_width = points[j].len();
            let (weight, direction) = shortest_path(
                (
                    coordinate_transformation(
                        points[i][0][i_height - 1].0,
                        points[i][0][i_height - 1].1,
                        direction_radians,
                    ),
                    coordinate_transformation(
                        points[i][i_width - 1][0].0,
                        points[i][i_width - 1][0].1,
                        direction_radians,
                    ),
                ),
                (
                    coordinate_transformation(
                        points[j][0][j_height - 1].0,
                        points[j][0][j_height - 1].1,
                        direction_radians,
                    ),
                    coordinate_transformation(
                        points[j][j_width - 1][0].0,
                        points[j][j_width - 1][0].1,
                        direction_radians,
                    ),
                ),
            );

            weights[i][j] = Some((weight, direction.clone()));
            weights[j][i] = Some((weight.abs(), direction.opposite().clone()));
        }
    }
    let mut last_row: Vec<Option<(f64, Direction)>> = vec![None; region_count + 1];
    for i in 0..region_count {
        let i_height = points[i][0].len();
        let i_width = points[i].len();
        let (weight, direction) = shortest_path(
            (
                coordinate_transformation(start_point.0, start_point.1, direction_radians),
                coordinate_transformation(start_point.0, start_point.1, direction_radians),
            ),
            (
                coordinate_transformation(
                    points[i][0][i_height - 1].0,
                    points[i][0][i_height - 1].1,
                    direction_radians,
                ),
                coordinate_transformation(
                    points[i][i_width - 1][0].0,
                    points[i][i_width - 1][0].1,
                    direction_radians,
                ),
            ),
        );
        last_row[i] = Some((weight, direction.clone()));
        weights[i].push(Some((weight.abs(), direction.opposite().clone())));
    }
    weights.push(last_row);

    println!("Weights {:?}", weights);

    for region_points in points.clone() {
        // Check if the input vector is rectangular
        let height = region_points[0].len();
        let width = region_points.len();

        if region_points.iter().any(|row| row.len() != height) {
            return Err(String::from("Input vector is not rectangular"));
        }

        if width < 2 {
            multiple_region_result.push(region_points.into_iter().flatten().collect());
            break;
        }

        let mut region_result: Vec<(f64, f64)> = Vec::new();

        for i in 0..width {
            region_result.push(region_points[i][0]);
        }

        for j in 1..height {
            region_result.push(region_points[width - 1][j]);
        }

        let ramps = (width - 2) / 2;

        for n in 0..ramps {
            for j in (1..height).rev() {
                region_result.push(region_points[width - 2 - 2 * n][j]);
            }

            for j in 1..height {
                region_result.push(region_points[width - 3 - 2 * n][j]);
            }
        }

        if width % 2 == 1 {
            let coils = (height - 1) / 2;

            for n in 0..coils {
                region_result.push(region_points[1][height - 1 - n * 2]);
                region_result.push(region_points[0][height - 1 - n * 2]);
                region_result.push(region_points[0][height - 2 - n * 2]);
                region_result.push(region_points[1][height - 2 - n * 2]);
            }

            if height % 2 == 0 {
                region_result.push(region_points[1][1]);
                region_result.push(region_points[0][1]);
            }
        } else {
            for j in (1..height).rev() {
                region_result.push(region_points[0][j]);
            }
        }
        multiple_region_result.push(region_result)
    }
    Ok(multiple_region_result.into_iter().flatten().collect())
}

#[derive(Debug, Clone)]
enum Direction {
    U,
    D,
    L,
    R,
    UL,
    UR,
    DL,
    DR,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::U => Direction::D,
            Direction::D => Direction::U,
            Direction::L => Direction::R,
            Direction::R => Direction::L,
            Direction::UL => Direction::DR,
            Direction::UR => Direction::DL,
            Direction::DL => Direction::UR,
            Direction::DR => Direction::UL,
        }
    }
}

fn find_direction(a: ((f64, f64), (f64, f64)), b: ((f64, f64), (f64, f64))) -> Direction {
    let ((a_left, a_top), (a_right, a_bottom)) = a;
    let ((b_left, b_top), (b_right, b_bottom)) = b;

    if b_right < a_left {
        if a_top < b_bottom {
            Direction::UL
        } else if b_top < a_bottom {
            Direction::DL
        } else {
            Direction::L
        }
    } else if a_right < b_left {
        if a_top < b_bottom {
            Direction::UR
        } else if b_top < a_bottom {
            Direction::DR
        } else {
            Direction::R
        }
    } else {
        if a_top < b_bottom {
            Direction::U
        } else {
            Direction::D
        }
    }
}

// Method to calculate the shortest path between two rectangles
fn shortest_path(a: ((f64, f64), (f64, f64)), b: ((f64, f64), (f64, f64))) -> (f64, Direction) {
    let direction = find_direction(a, b);

    let ((a_left, a_top), (a_right, a_bottom)) = a;
    let ((b_left, b_top), (b_right, b_bottom)) = b;

    (
        match direction {
            Direction::U => b_bottom - a_top,
            Direction::D => a_bottom - b_top,
            Direction::L => a_left - b_right,
            Direction::R => b_left - a_right,
            // 0011 1011
            // 0001 1001
            Direction::UL => euclidean_distance(&(a_top, a_left), &(b_bottom, b_right)),
            Direction::UR => euclidean_distance(&(a_top, a_right), &(b_bottom, b_left)),
            Direction::DL => euclidean_distance(&(a_bottom, a_left), &(b_top, b_right)),
            Direction::DR => euclidean_distance(&(a_bottom, a_right), &(b_top, b_left)),
        },
        direction,
    )
}
