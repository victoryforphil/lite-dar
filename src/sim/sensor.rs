use log::info;
use nalgebra::{point, vector};
use rapier3d::prelude::{QueryFilter, Ray};

use crate::types::pose::Pose;

use super::context::SimulationContext;
#[derive(Clone)]
pub struct TOFSensor {
    pub pose: Pose,
}
#[derive(Clone)]
pub struct TOFSensorResult {
    pub result: Vec<Vec<f32>>,
}
impl TOFSensor {
    pub fn new() -> Self {
        Self { pose: Pose::zero() }
    }

    pub fn init(&mut self, context: &mut SimulationContext) {
        self.pose.position.x = -10.0;
        self.pose.position.y = 1.0;

        self.pose.position.z = 0.0;
    }

    pub fn tick(&mut self, context: &mut SimulationContext) -> TOFSensorResult {
        let max_toi = 100.0;
        let solid = true;
        let filter = QueryFilter::new();

        // Store 2d array of lidar distances
        let lidar_w = 32;
        let lidar_h = 32;
        let mut lidar_distances = vec![vec![0.0; lidar_w as usize]; lidar_h as usize];

        for y in 0..lidar_w {
            for x in 0..lidar_h {
                let ray = self.calculate_ray(x, y, lidar_w, lidar_h, 120.0);
                let hit_result = context.query_pipeline.cast_ray(
                    &context.rigid_bodies,
                    &context.coliders,
                    &ray,
                    max_toi,
                    solid,
                    filter,
                );

                match hit_result {
                    Some(hit) => {
                        lidar_distances[x as usize][y as usize] = 1.0;
                        info!("Hit: {:?}", hit)
                    }
                    None => {
                        lidar_distances[x as usize][y as usize] = 0.0;
                    }
                }
            }
        }

        TOFSensorResult {
            result: lidar_distances,
        }
    }

    // Calculate Ray from a step, max step, total FOV
    pub fn calculate_ray(&self, x: u16, y: u16, w: u16, h: u16, fov: f32) -> Ray {
        let x = x as f32;
        let y = y as f32;
        let w = w as f32;
        let h = h as f32;
        let fov = fov.to_radians();

        let x = (x / w) * fov - fov / 2.0;
        let y = (y / h) * fov - fov / 2.0;

        let dir = vector![x, y, 0.0];

        let ray = Ray::new(point![0.0, 1.0, 0.0], dir);

        ray
    }
}
