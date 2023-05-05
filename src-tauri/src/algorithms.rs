use tauri::command;

use std::f64::{INFINITY, NEG_INFINITY};

#[tauri::command]
pub fn discretize_area(polygon: Vec<(f64, f64)>, photo_width: f64, photo_height: f64) -> Result<Vec<(f64, f64)>, String> {
    println!("Received polygon coordinates: {:?}", polygon);

    let (mut min_x, mut max_x, mut min_y, mut max_y) = (INFINITY, NEG_INFINITY, INFINITY, NEG_INFINITY);

    for (x, y) in &polygon {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }

    fn is_inside_polygon(point: (f64, f64), polygon: &Vec<(f64, f64)>) -> bool {
        let mut inside = false;
        let len = polygon.len();
        let mut j = len - 1;
        for i in 0..len {
            let (x_i, y_i) = polygon[i];
            let (x_j, y_j) = polygon[j];

            let intersect = (y_i > point.1) != (y_j > point.1)
                && point.0 < (x_j - x_i) * (point.1 - y_i) / (y_j - y_i) + x_i;
            if intersect {
                inside = !inside;
            }
            j = i;
        }
        inside
    }

    let mut result = Vec::new();
    let (half_camera_width, half_camera_height) = (photo_width / 2.0, photo_height / 2.0);
    let mut x = min_x;

    while x <= max_x {
        let mut y = min_y;

        while y <= max_y {
            let corners = vec![
                (x, y),
                (x + photo_width, y),
                (x, y + photo_height),
                (x + photo_width, y + photo_height),
            ];

            let is_any_corner_inside = corners
                .into_iter()
                .any(|corner| is_inside_polygon(corner, &polygon));

            if is_any_corner_inside {
                let center_x = x + half_camera_width;
                let center_y = y + half_camera_height;
                result.push((center_x, center_y));
            }

            y += photo_height;
        }

        x += photo_width;
    }

    Ok(result)
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
        result.push(nearest_point);
        current_point = nearest_point;
    }

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

// pub fn brute_force(points: &[(f64, f64)], start_point: &(f64, f64)) -> Vec<(f64, f64)> {
//     // Implement the brute_force algorithm
// }
