use nalgebra::{point, vector};
use rapier3d::prelude::{QueryFilter, Ray};

use crate::types::pose::Pose;

use super::context::SimulationContext;

pub struct TOFSensor {
    pub pose: Pose,
}
impl TOFSensor {
    pub fn new() -> Self {
        Self { pose: Pose::zero() }
    }

    pub fn init(&mut self, context: &mut SimulationContext) {
        self.pose.position.x = -10.0;
        self.pose.position.y = 0.0;

        self.pose.position.z = 0.5;
    }

    pub fn tick(&mut self, context: &mut SimulationContext) {
        let ray = Ray::new(point![1.0, 2.0, 3.0], vector![0.0, 1.0, 0.0]);
        let max_toi = 4.0;
        let solid = true;
        let filter = QueryFilter::default();

        if let Some((handle, toi)) =
            query_pipeline.cast_ray(&collider_set, &ray, max_toi, solid, filter)
        {
            // The first collider hit has the handle `handle` and it hit after
            // the ray travelled a distance equal to `ray.dir * toi`.
            let hit_point = ray.point_at(toi); // Same as: `ray.origin + ray.dir * toi`
            println!("Collider {:?} hit at point {}", handle, hit_point);
        }
    }
}
