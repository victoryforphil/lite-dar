use nalgebra::vector;
use rapier3d::prelude::{ColliderBuilder, RigidBodyBuilder};

use super::context::SimulationContext;

pub struct SimBall {}

impl SimBall {
    pub fn new() -> Self {
        Self {}
    }
    pub fn create(&mut self, context: &mut SimulationContext) {
        //Create floor
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![5.0, 10.0, 0.0])
            .build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let ball_body_handle = context.rigid_bodies.insert(rigid_body);
        context
            .coliders
            .insert_with_parent(collider, ball_body_handle, &mut context.rigid_bodies);
    }
}
