pub mod controller;
pub mod gravity;
pub mod particle;
pub mod sand;
pub mod softbody;

pub use controller::{Controller, SimulationBounds};
pub use gravity::GravityController;

pub type Vector2D = nalgebra::Vector2<f32>;

pub mod constants
{
    use std::time::Duration;
    pub const DT: Duration = Duration::from_millis(50);
}
