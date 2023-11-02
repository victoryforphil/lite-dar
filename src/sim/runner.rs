use std::{
    sync::mpsc::{channel, Sender},
    thread::{self, JoinHandle},
    time::Duration,
};

use super::{Simulation, SimulationState};

/// Simulation runner that creates a thread and channel, starts a simulation,
/// Calls simulation tick on a loop, and sends the simulation state to the
/// channel during each tick.
///
///
/// It creates the simulation and calls init() on new(), but runs the loop once start() is called.

pub struct SimRunner {
    pub simulation: Simulation,
    pub channel: Sender<SimulationState>,
    pub thread: Option<JoinHandle<()>>,
}

impl SimRunner {}
