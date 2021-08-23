use super::{
    controller::{default_fetch_parameters, Controller, SimulationBounds},
    particle::{get_two_particles, Particle, Vector2D},
};
use sdl2::gfx::primitives::{DrawRenderer, ToColor};

use std::time::Duration;

#[derive(Default)]
pub struct GravityController
{
    particles: Vec<Particle>,
}

impl GravityController
{
    pub fn calculate_forces(&mut self, index1: usize, index2: usize, dt: &Duration)
    {
        let (p1, p2) = get_two_particles(&mut self.particles, index1, index2);
        let distance = p1.position - p2.position;
        let distance_normalized = distance.normalize();
        let mut gravity_force =
            ((p1.mass * p2.mass) / distance.norm_squared()) * distance_normalized;

        p2.apply_force(&gravity_force);
        gravity_force *= -1.0;
        p1.apply_force(&gravity_force);
        Particle::handle_collision(p1, p2, dt);
    }
}

impl Controller for GravityController
{
    fn name(&self) -> &'static str
    {
        "Gravity"
    }

    fn tick(&mut self, dt: &Duration, bounds: &SimulationBounds)
    {
        if self.particles.len() < 1
        {
            return;
        }
        for i in 0..self.particles.len() - 1
        {
            for j in (i + 1)..self.particles.len()
            {
                self.calculate_forces(i, j, dt);
            }
        }

        Particle::update_vec(&mut self.particles, dt);
    }

    fn fetch_parameters_from_input(&mut self, bounds: &SimulationBounds)
    {
        self.particles = default_fetch_parameters(bounds);
    }

    fn render(&self, canvas: &sdl2::render::Canvas<sdl2::video::Window>)
    {
        let mut mass_center = Vector2D::zeros();
        let mut mass_sum = 0.0;
        self.particles.iter().for_each(|p| {
            p.render(canvas, sdl2::pixels::Color::RGB(0, 0, 255));
            mass_center += p.mass * p.position;
            mass_sum += p.mass;
        });
        mass_center /= mass_sum;
        canvas
            .filled_circle(
                mass_center.x as i16,
                mass_center.y as i16,
                3,
                sdl2::pixels::Color::RED,
            )
            .unwrap();
    }
}
