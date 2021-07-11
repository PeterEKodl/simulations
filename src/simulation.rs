pub mod controller;
pub mod particle;
pub mod gravity;
pub mod sand;
pub mod softbody;


pub type Vector2D = nalgebra::Vector2<f32>;

struct Simulation
{
    contoller: Box<dyn controller::Controller>

}


pub mod constants
{
    use std::time::Duration;
    const DT: Duration = Duration::from_millis(50);
    
}
