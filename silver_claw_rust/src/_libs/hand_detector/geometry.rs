#![allow(dead_code)]

use std::f64::consts::PI;

/**
 * Calculates the dot product for 2 vectors.
 */
pub fn dot_product(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/**
 * Calculates the magnitude of a vector.
 */
pub fn magnitude(a: &Vec<f32>) -> f32 {
    a.iter().map(|i| i * i).sum::<f32>().sqrt()
}

/**
 * Calculates the angle between 2 vectors.
 */
pub fn compute_angle(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    (dot_product(a, b) / (magnitude(a) * magnitude(b))).acos() / (2f32 * PI as f32) * 360f32
}

/**
 * Calculate a vector between 2 points.
 */
pub fn compute_vec_from_points(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
    let mut v: Vec<f32> = Vec::new();
    for element in a.iter().zip(b.iter()).map(|(x, y)| y - x) {
        v.push(element);
    }
    v
}
