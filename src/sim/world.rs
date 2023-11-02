use nalgebra::vector;
use rapier3d::prelude::ColliderBuilder;

use super::context::SimulationContext;

pub struct World {}

impl World {
    pub fn new() -> Self {
        Self {}
    }
    pub fn create(&mut self, context: &mut SimulationContext) {
        //Create floor
        let floor_colider = ColliderBuilder::cuboid(100.0, 100.0, 0.1);

        let cube_colider = ColliderBuilder::cuboid(1.0, 1.0, 1.0)
            .translation(vector![5.0, 0.0, 3.0])
            .build();
        context.coliders.insert(floor_colider);
        context.coliders.insert(cube_colider);
    }
}
