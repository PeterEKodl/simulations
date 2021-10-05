use super::{
    controller::{Controller, SimulationBounds},
    particle::{Particle, Vector2D},
};
use std::time::Duration;

pub struct SoftbodyController
{
    particles: Vec<Particle>,
    edge_lengths: Vec<f32>,
    k: f32,
}

impl SoftbodyController
{
    fn calculate_force((neighbor, node): (&mut Particle, &mut Particle), d: f32, k: f32)
    {
        use super::constants::DAMPING;
        let distance = neighbor.position - node.position;
        let deformation = d - distance.norm();
        let direction = distance.normalize();
        let force = direction * k * deformation;
        neighbor.apply_force(&force);
        node.apply_force(&(-force));

        let relative_velocity = node.velocity - neighbor.velocity;
        let force = -*DAMPING * relative_velocity.dot(&direction) * direction;
        node.apply_force(&force);
        neighbor.apply_force(&-force);
    }
}

impl Default for SoftbodyController
{
    fn default() -> Self
    {
        Self {
            particles: Vec::new(),
            edge_lengths: Vec::new(),
            k: 0.0,
        }
    }
}

impl Controller for SoftbodyController
{
    fn name(&self) -> &'static str
    {
        "Softbody"
    }

    fn tick(&mut self, dt: &Duration, bounds: &SimulationBounds)
    {
        use super::constants::GRAVITY;
        static GRAVITY_VECTOR: Vector2D = Vector2D::new(0.0, GRAVITY);

        let mut edge_length_iter = self.edge_lengths.iter();
        for i in 0..self.particles.len() - 1
        {
            let (lower, upper) = self.particles.split_at_mut(i + 1);
            let p1 = &mut lower[i];
            for p2 in upper
            {
                Self::calculate_force((p2, p1), *edge_length_iter.next().unwrap(), self.k);
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
        const RADIUS: f32 = 20.0;
        const MASS: f32 = 100.0;
        use std::str::FromStr;
        let mut input = String::new();
        let input_max = bounds.0.min(bounds.1) - RADIUS * 2.0;
        let input_min = RADIUS * 2.0;
        let side_length = loop
        {
            input.clear();
            println!(
                "Distance between soft body nodes. [{}; {}]",
                input_min, input_max
            );
            if std::io::stdin().read_line(&mut input).is_err()
            {
                println!("Input error.");
                continue;
            }
            let value = if let Ok(value) = f32::from_str(input.trim())
            {
                value
            }
            else
            {
                println!("Input must be a real number.");
                continue;
            };
            if !(input_min..=input_max).contains(&value)
            {
                println!("Input must be in range [{}; {}]", input_min, input_max);
                continue;
            }
            break value;
        };
        self.k = loop
        {
            input.clear();
            println!("Spring stiffness:");
            if std::io::stdin().read_line(&mut input).is_err()
            {
                println!("Input error.");
                continue;
            }
            let value = if let Ok(value) = f32::from_str(input.trim())
            {
                value
            }
            else
            {
                println!("Input must be a real number.");
                continue;
            };

            break value;
        };
        let offset = (
            (bounds.0 - side_length) / 2.0,
            (bounds.1 - side_length) / 2.0,
        );

        for i in 0..=1
        {
            for j in 0..=1
            {
                self.particles.push(Particle::new(
                    MASS,
                    RADIUS,
                    Vector2D::new(
                        offset.0 + j as f32 * side_length,
                        offset.1 + i as f32 * side_length,
                    ),
                ));
            }
        }

        for i in 0..self.particles.len() - 1
        {
            let (lower, upper) = self.particles.split_at(i + 1);
            let p1 = &lower[i];
            for p2 in upper
            {
                self.edge_lengths.push((p1.position - p2.position).norm());
            }
        }
    }

    fn render(&self, canvas: &sdl2::render::Canvas<sdl2::video::Window>)
    {
        self.particles
            .iter()
            .for_each(|p| p.render(canvas, sdl2::pixels::Color::RED));
    }
}
