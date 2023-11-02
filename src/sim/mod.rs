use log::info;

use self::{ball::SimBall, context::SimulationContext, sensor::TOFSensorResult, world::World};

pub mod ball;
pub mod context;
pub mod runner;
pub mod sensor;
pub mod world;
pub struct Simulation {
    pub context: SimulationContext,
    pub state: SimulationState,
    pub world: World,
    pub sensors: Vec<sensor::TOFSensor>,
    pub ball: SimBall,
    pub initd: bool,
}

pub struct SimulationState {
    pub tof_results: Vec<TOFSensorResult>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            context: SimulationContext::new(),
            state: SimulationState {
                tof_results: vec![],
            },
            world: World::new(),
            sensors: vec![sensor::TOFSensor::new()],
            ball: SimBall::new(),
            initd: false,
        }
    }

    pub fn init(&mut self) {
        self.world.create(&mut self.context);
        for sensor in self.sensors.iter_mut() {
            sensor.init(&mut self.context);
        }

        self.ball.create(&mut self.context);
        self.initd = true;
    }

    pub fn tick(&mut self) -> &SimulationState {
        info!("tick");
        println!("tick again");
        self.context.physics_pipeline.step(
            &self.context.gravity,
            &self.context.intergration_parameters,
            &mut self.context.island_manager,
            &mut self.context.broad_phase,
            &mut self.context.narrow_phase,
            &mut self.context.rigid_bodies,
            &mut self.context.coliders,
            &mut self.context.impulse_joint_set,
            &mut self.context.multibody_joint_set,
            &mut self.context.ccd_solver,
            Some(&mut self.context.query_pipeline),
            &self.context.physics_hooks,
            &self.context.ev,
        );
        self.state.tof_results = vec![];
        for sensor in self.sensors.iter_mut() {
            let result = sensor.tick(&mut self.context);
            self.state.tof_results.push(result);
        }
        return &self.state;
    }
}
