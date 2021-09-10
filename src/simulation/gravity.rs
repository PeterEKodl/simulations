use super::{
    controller::{default_fetch_parameters, Controller, SimulationBounds},
    particle::{get_two_particles, Particle, Vector2D},
};
use sdl2::gfx::primitives::DrawRenderer;

use std::time::Duration;

#[derive(Default)]
pub struct GravityController
{
    particles: Vec<Particle>,
    scale: f32,
    bounds: SimulationBounds,
}

impl GravityController
{
    fn calculate_forces(&mut self, index1: usize, index2: usize, dt: &Duration)
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

    fn transform_position(&self, mut x: f32, mut y: f32) -> (i16, i16)
    {
        x += self.bounds.0 * (self.scale - 1.0) / 2.0;
        y += self.bounds.1 * (self.scale - 1.0) / 2.0;
        ((x / self.scale) as i16, (y / self.scale) as i16)
    }
}

impl Controller for GravityController
{
    fn name(&self) -> &'static str
    {
        "Gravity"
    }

    fn tick(&mut self, dt: &Duration, _bounds: &SimulationBounds)
    {
        if self.particles.is_empty()
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
        self.scale = 1.0;
        self.bounds = bounds.clone();
    }

    fn render(&self, canvas: &sdl2::render::Canvas<sdl2::video::Window>)
    {
        let mut mass_center = Vector2D::zeros();
        let mut mass_sum = 0.0;
        self.particles.iter().for_each(|p| {
            let position = self.transform_position(p.position.x, p.position.y);
            canvas
                .filled_circle(
                    position.0,
                    position.1,
                    (p.radius / self.scale) as i16,
                    sdl2::pixels::Color::BLUE,
                )
                .unwrap();
            mass_center += p.mass * p.position;
            mass_sum += p.mass;
        });
        mass_center /= mass_sum;
        let position = self.transform_position(mass_center.x, mass_center.y);
        canvas
            .filled_circle(position.0, position.1, 3, sdl2::pixels::Color::RED)
            .unwrap();
    }

    fn handle_key_down(&mut self, key: sdl2::keyboard::Keycode)
    {
        const SCALE_RATE: f32 = 1.05;
        use sdl2::keyboard::Keycode;
        match key
        {
            Keycode::Equals => self.scale /= SCALE_RATE,
            Keycode::Minus => self.scale *= SCALE_RATE,
            Keycode::Num0 => self.scale = 1.0,
            _ =>
            {}
        }
    }
}
