use self::{context::SimulationContext, world::World};

pub mod context;
pub mod sensor;
pub mod world;

pub struct Simulation {
    pub context: SimulationContext,
    pub world: World,
    pub sensors: Vec<sensor::TOFSensor>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            context: SimulationContext::new(),
            world: World::new(),
            sensors: vec![sensor::TOFSensor::new()],
        }
    }

    pub fn init(&mut self) {
        self.world.create(&mut self.context);
        for sensor in self.sensors.iter_mut() {
            sensor.init(&mut self.context);
        }
    }

    pub fn tick(&mut self) {
        for sensor in self.sensors.iter_mut() {
            sensor.tick(&mut self.context);
        }
    }
}
