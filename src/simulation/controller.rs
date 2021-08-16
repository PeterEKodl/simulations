use std::time::Duration;

#[derive(Default)]
pub struct SimulationBounds(pub f32, pub f32);

pub trait Controller
{
    fn tick(&mut self, dt: &Duration, bounds: &SimulationBounds);
    fn name(&self) -> &'static str;
    // bounds is required in order to configure the location of particles
    fn fetch_parameters_from_input(&mut self, bounds: &SimulationBounds);
    fn render(&self, canvas: &sdl2::render::Canvas<sdl2::video::Window>);
}
