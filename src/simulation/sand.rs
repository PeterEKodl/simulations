use super::{
    controller::{default_fetch_parameters, Controller, SimulationBounds},
    particle::{get_two_particles, Particle, Vector2D},
};
use std::time::Duration;

#[derive(Default)]
pub struct SandController
{
    particles: Vec<Particle>,
}

impl Controller for SandController
{
    fn name(&self) -> &'static str
    {
        "Sand"
    }

    fn tick(&mut self, dt: &Duration, bounds: &SimulationBounds)
    {
        use super::constants::GRAVITY;
        static GRAVITY_VECTOR: Vector2D = Vector2D::new(0.0, GRAVITY);
        if self.particles.len() < 1
        {
            return;
        }
        for i in 0..self.particles.len() - 1
        {
            for j in (i + 1)..self.particles.len()
            {
                let (p1, p2) = get_two_particles(&mut self.particles, i, j);
                Particle::handle_collision(p1, p2, dt);
            }
        }
        self.particles.iter_mut().for_each(|p| {
            p.apply_acceleration(&GRAVITY_VECTOR);
            p.wall_collisions(dt, bounds);
        });
        Particle::update_vec(&mut self.particles, dt);
    }

    fn fetch_parameters_from_input(&mut self, bounds: &SimulationBounds)
    {
        self.particles = default_fetch_parameters(bounds);
    }

    fn render(&self, canvas: &sdl2::render::Canvas<sdl2::video::Window>)
    {
        self.particles
            .iter()
            .for_each(|p| p.render(canvas, sdl2::pixels::Color::RGB(0, 0, 255)));
    }
}
