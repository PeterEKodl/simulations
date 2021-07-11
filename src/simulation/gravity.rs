use super::controller::Controller;
use super::particle::Particle;
use super::particle::get_two_particles;
use std::time::Duration;

pub struct GravityController
{
   particles: Vec<Particle> 
}

impl GravityController
{
    pub fn calculate_forces(&mut self, index1: usize, index2: usize, dt: &Duration)
    {
        let (p1, p2) = get_two_particles(&mut self.particles, index1, index2);
        let distance = p1.position - p2.position;
        let distance_normalized = distance.normalize();
        let mut gravity_force = ( (p1.mass * p2.mass)/distance.norm_squared() ) * distance_normalized;

        p2.apply_force(&gravity_force);
        gravity_force *= -1.0;
        p1.apply_force(&gravity_force);

        if (p1.radius + p2.radius).powi(2) > distance.norm_squared()
        {
            let K = (p1.mass + p2.mass)/(dt.as_secs_f32()); 
            let overlap = (p1.radius + p2.radius) - distance.norm();

            p1.apply_force(&(distance_normalized * K * overlap));
            p2.apply_force(&(-distance_normalized * K * overlap));

        }

    }  

}

impl Controller for GravityController
{
    
    fn name(&self) -> &'static str
    {
        "Gravity"
    }

    fn tick(&mut self, dt: &Duration)
    {
        for i in 1..self.particles.len()-1
        {
            for j in i+1..self.particles.len()
            {
                
            }
        }
    }

    fn fetch_parameters_from_input(&mut self)
    {
    }

    
}

