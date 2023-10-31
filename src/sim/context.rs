use nalgebra::Vector3;
use rapier3d::prelude::{
    BroadPhase, CCDSolver, ColliderSet, ImpulseJointSet, IntegrationParameters, IslandManager,
    MultibodyJointSet, NarrowPhase, PhysicsPipeline, RigidBodySet,
};

pub struct SimulationContext {
    pub rigid_bodies: RigidBodySet,
    pub coliders: ColliderSet,
    pub gravity: Vector3<f32>,
    pub intergration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
}

impl SimulationContext {
    pub fn new() -> Self {
        Self {
            rigid_bodies: RigidBodySet::new(),
            coliders: ColliderSet::new(),
            gravity: Vector3::new(0.0, 0.0, -9.81),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context() {
        let context = SimulationContext::new();
        assert_eq!(context.rigid_bodies.len(), 0);
        assert_eq!(context.coliders.len(), 0);
    }
}
