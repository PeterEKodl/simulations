use super::controller::SimulationBounds;
pub use super::Vector2D;
use rayon::prelude::*;
use sdl2::gfx::primitives::{DrawRenderer, ToColor};
use std::time::Duration;
pub struct Particle
{
    pub mass: f32,
    pub radius: f32,
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub force_accumulator: Vector2D,
}

impl Particle
{
    pub fn new(mass: f32, radius: f32, position: Vector2D) -> Self
    {
        Self {
            mass,
            radius,
            position,
            velocity: Vector2D::zeros(),
            force_accumulator: Vector2D::zeros(),
        }
    }

    pub fn apply_force(&mut self, force: &Vector2D)
    {
        self.force_accumulator += force;
    }

    pub fn apply_acceleration(&mut self, acceleration: &Vector2D)
    {
        self.force_accumulator += acceleration * self.mass;
    }

    pub fn tick(&mut self, dt: &Duration)
    {
        self.velocity += (self.force_accumulator / self.mass) * dt.as_secs_f32();
        self.position += self.velocity * dt.as_secs_f32();
        self.force_accumulator.fill(0.0);
    }

    pub fn update_vec(particles: &mut Vec<Particle>, dt: &Duration)
    {
        particles.par_iter_mut().for_each(|p| p.tick(dt));
    }

    pub fn render<C: ToColor>(&self, canvas: &sdl2::render::Canvas<sdl2::video::Window>, color: C)
    {
        canvas
            .filled_circle(
                self.position.x as i16,
                self.position.y as i16,
                self.radius as i16,
                color,
            )
            .unwrap();
    }

    pub fn handle_collision(p1: &mut Particle, p2: &mut Particle, dt: &Duration)
    {
        use super::constants::DAMPING;
        let distance = p1.position - p2.position;
        if (p1.radius + p2.radius).powi(2) > distance.norm_squared()
        {
            let distance_normalized = distance.normalize();
            let k = (p1.mass + p2.mass) / (dt.as_secs_f32());
            let overlap = (p1.radius + p2.radius) - distance.norm();

            p1.apply_force(&(distance_normalized * k * overlap));
            p2.apply_force(&(-distance_normalized * k * overlap));
            let relative_velocity = p1.velocity - p2.velocity;
            let force = -*DAMPING * relative_velocity;
            p1.apply_force(&force);
            p2.apply_force(&-force);
        }
    }

    pub fn wall_collisions(&mut self, dt: &Duration, bounds: &SimulationBounds)
    {
        use super::constants::DAMPING;
        // Floor collision
        if self.position.y + self.radius > bounds.1
        {
            let normal: Vector2D = -Vector2D::y();
            let displacement = self.position.y + self.radius - bounds.1;
            let force = self.mass / dt.as_secs_f32().powi(2) * displacement;
            self.apply_force(&(force * normal));
            self.apply_force(&(-*DAMPING * self.velocity));
        }
        // Ceiling collision
        if self.position.y - self.radius < 0.0
        {
            let normal: Vector2D = Vector2D::y();
            let displacement = -self.position.y + self.radius;
            let force = self.mass / dt.as_secs_f32().powi(2) * displacement;
            self.apply_force(&(force * normal));
            self.apply_force(&(-*DAMPING * self.velocity));
        }
        // Right side collision
        if self.position.x + self.radius > bounds.0
        {
            let normal: Vector2D = -Vector2D::x();
            let displacement = self.position.x + self.radius - bounds.0;
            let force = self.mass / dt.as_secs_f32().powi(2) * displacement;
            self.apply_force(&(force * normal));
            self.apply_force(&(-*DAMPING * self.velocity));
        }
        // Left side collision
        if self.position.x - self.radius < 0.0
        {
            let normal: Vector2D = Vector2D::x();
            let displacement = -self.position.x + self.radius;
            let force = self.mass / dt.as_secs_f32().powi(2) * displacement;
            self.apply_force(&(force * normal));
            self.apply_force(&(-*DAMPING * self.velocity));
        }
    }
}

// Fetches two mutable references to two different elements in the vector by index.
pub fn get_two_particles(
    particles: &mut Vec<Particle>,
    index1: usize,
    index2: usize,
) -> (&mut Particle, &mut Particle)
{
    let split = particles.split_at_mut(index1.max(index2));
    (&mut split.0[index1.min(index2)], &mut split.1[0])
}
