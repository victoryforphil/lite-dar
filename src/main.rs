use std::{thread, time::Duration};

pub mod sim;
pub mod types;
pub fn main() {
    let mut simulation = sim::Simulation::new();

    simulation.init();

    for _ in 0..100 {
        simulation.tick();
        // sleep 1ms

        thread::sleep(Duration::from_millis(1));
    }
}
