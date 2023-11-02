use std::{thread, time::Duration};

pub mod sim;
pub mod types;
pub mod viz;
pub fn main() {
    env_logger::init();
    let mut viz = viz::SimViz::new();
}
