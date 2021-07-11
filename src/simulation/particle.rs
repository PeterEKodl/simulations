use std::time::Duration;
use super::Vector2D;

pub struct Particle
{
    pub mass: f32,
    pub radius: f32,
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub force: Vector2D
    
}

impl Particle
{
    pub fn apply_force(&mut self, force: &Vector2D)
    {
        self.force += force;
    }

    pub fn apply_acceleration(&mut self, acceleration: &Vector2D)
    {
        self.force += acceleration * self.mass;
    }

    pub fn tick(&mut self, dt: &Duration)
    {
        self.velocity += (self.force/self.mass) * dt.as_secs_f32();
        self.position += self.velocity * dt.as_secs_f32();
        self.force.fill(0.0);
    }

}

// Fetches two mutable references to two different elements in the vector by index.
pub fn get_two_particles(particles: &mut Vec<Particle>, index1: usize, index2: usize) -> (&mut Particle, &mut Particle)
{
    let split = particles.split_at_mut(index1.max(index2));
    (&mut split.0[index1.min(index2)], &mut split.1[0])
}
