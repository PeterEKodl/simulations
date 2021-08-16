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
    use lazy_static::lazy_static;
    use std::time::Duration;
    pub const DT: Duration = Duration::from_millis(50);
    const RESTITUTION: f32 = 0.1;
    lazy_static! {
        pub static ref DAMPING: f32 =
            -RESTITUTION.ln() * (std::f32::consts::PI.powi(2) + RESTITUTION.ln().powi(2)).sqrt();
    }
}
