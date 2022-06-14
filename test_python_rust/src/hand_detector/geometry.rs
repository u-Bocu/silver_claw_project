#![allow(dead_code)]

use std::f64::consts::PI;

pub fn dot_product(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

pub fn magnitude(a: &Vec<f32>) -> f32 {
    a.iter().map(|i| i * i).sum::<f32>().sqrt()
}

pub fn compute_angle(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    (dot_product(a, b) / (magnitude(a) * magnitude(b))).acos() / (2f32 * PI as f32) * 360f32
}

pub fn compute_vec_from_points(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
    let mut v: Vec<f32> = Vec::new();
    for element in a.iter().zip(b.iter()).map(|(x, y)| y - x) {
        v.push(element);
    }
    v
}
