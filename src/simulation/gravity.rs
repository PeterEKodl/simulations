use super::controller::{Controller, SimulationBounds};
use super::particle::{get_two_particles, Particle, Vector2D};
use rand::{distributions::Uniform, prelude::*};

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
        let mut input_choice = String::new();
        println!("Body count:");

        let body_count = loop
        {
            input_choice.clear();
            if let Err(_) = std::io::stdin().read_line(&mut input_choice)
            {
                println!("Input error.");
                continue;
            }
            let value = if let Ok(value) = usize::from_str_radix(input_choice.trim(), 10)
            {
                value
            }
            else
            {
                println!("Input must be integer.");
                continue;
            };

            break value;
        };

        const RADIUS: f32 = 10.0;
        const MASS: f32 = 1000.0;

        let grid_n = (body_count as f32).sqrt().ceil() as i32;
        let grid_distance = (
            bounds.0 / ((grid_n + 2) as f32),
            bounds.1 / ((grid_n + 2) as f32),
        );

        let rand_ranges = (
            Uniform::from(-(grid_distance.0 / 2.0 - RADIUS)..(grid_distance.0 / 2.0 - RADIUS)),
            Uniform::from(-(grid_distance.1 / 2.0 - RADIUS)..(grid_distance.1 / 2.0 - RADIUS)),
        );

        let mut x = 0;
        let mut y = 0;
        let mut rng = thread_rng();

        for _ in 0..body_count
        {
            if x == grid_n
            {
                x = 0;
                y += 1;
            }
            self.particles.push(Particle::new(
                MASS,
                RADIUS,
                Vector2D::new(
                    ((x + 1) as f32) * grid_distance.0 + rand_ranges.0.sample(&mut rng),
                    ((y + 1) as f32) * grid_distance.1 + rand_ranges.1.sample(&mut rng),
                ),
            ));
            x += 1;
        }
    }

    fn render(&self, canvas: &sdl2::render::Canvas<sdl2::video::Window>)
    {
        self.particles
            .iter()
            .for_each(|p| p.render(canvas, sdl2::pixels::Color::RGB(0, 0, 255)));
    }
}
