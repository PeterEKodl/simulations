use super::particle::Particle;
use super::Vector2D;
use rand::{distributions::Uniform, prelude::*};
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

pub fn default_fetch_parameters(bounds: &SimulationBounds) -> Vec<Particle>
{
    const RADIUS: f32 = 10.0;
    const MASS: f32 = 1000.0;

    let mut input_choice = String::new();
    println!("Body count:");

    let input_max = ((bounds.0 / (RADIUS * 2.0) - 1.0).floor())
        .min((bounds.1 / (RADIUS * 2.0) - 1.0).floor())
        .powi(2) as usize;
    println!("Max: {}", input_max);
    let body_count = loop
    {
        input_choice.clear();
        if std::io::stdin().read_line(&mut input_choice).is_err()
        {
            println!("Input error.");
            continue;
        }
        let value = if let Ok(value) = input_choice.trim().parse::<usize>()
        {
            value
        }
        else
        {
            println!("Input must be integer.");
            continue;
        };
        if value > input_max
        {
            println!("Input must be smaller than the max {}.", input_max);
            continue;
        }

        break value;
    };
    let grid_n = (body_count as f32).sqrt().ceil() as i32;
    let grid_distance = (
        bounds.0 / ((grid_n + 1) as f32),
        bounds.1 / ((grid_n + 1) as f32),
    );
    let rand_ranges = (
        -(grid_distance.0 / 2.0 - RADIUS)..=(grid_distance.0 / 2.0 - RADIUS),
        -(grid_distance.1 / 2.0 - RADIUS)..=(grid_distance.1 / 2.0 - RADIUS),
    );
    let distributions = (Uniform::from(rand_ranges.0), Uniform::from(rand_ranges.1));

    let mut x = 0;
    let mut y = 0;
    let mut rng = thread_rng();

    let mut particles = Vec::with_capacity(body_count);

    for _ in 0..body_count
    {
        if x == grid_n
        {
            x = 0;
            y += 1;
        }
        particles.push(Particle::new(
            MASS,
            RADIUS,
            Vector2D::new(
                ((x + 1) as f32) * grid_distance.0 + distributions.0.sample(&mut rng),
                ((y + 1) as f32) * grid_distance.1 + distributions.1.sample(&mut rng),
            ),
        ));
        x += 1;
    }
    particles
}
