use super::{
    controller::{Controller, SimulationBounds},
    particle::{get_two_particles, Particle, Vector2D},
};
use std::time::Duration;

pub struct SoftbodyController
{
    particles: Vec<Particle>,
    side_length: f32,
    diagonal_length: f32,
    k: f32,
}

impl SoftbodyController
{
    fn calculate_force((neighbor, node): (&mut Particle, &mut Particle), d: f32, k: f32)
    {
        use super::constants::DAMPING;
        let distance = neighbor.position - node.position;
        let deformation = d - distance.norm();
        let mut direction = distance.normalize();
        let force = direction * k * deformation;
        neighbor.apply_force(&force);
        node.apply_force(&(-force));

        let mut relative_velocity = node.velocity - neighbor.velocity;
        node.apply_force(&(-*DAMPING * relative_velocity.dot(&direction) * direction));
        direction *= -1.0;
        relative_velocity *= -1.0;
        neighbor.apply_force(&(-*DAMPING * relative_velocity.dot(&direction) * direction));
    }
}

impl Default for SoftbodyController
{
    fn default() -> Self
    {
        Self {
            particles: Vec::new(),
            side_length: 0.0,
            diagonal_length: 0.0,
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

        // Stupid hack to iterate through the nodes and their neighbors.
        [[1, 2, 3], [-1, 3, 2], [3, -1, -1]]
            .iter()
            .enumerate()
            .for_each(|(e, t)| {
                for (index, &i) in t.iter().enumerate()
                {
                    if i != -1
                    {
                        Self::calculate_force(
                            get_two_particles(&mut self.particles, e, i as usize),
                            if index == 2
                            {
                                self.diagonal_length
                            }
                            else
                            {
                                self.side_length
                            },
                            self.k,
                        );
                    }
                }
            });

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
        self.side_length = side_length;
        self.diagonal_length = std::f32::consts::SQRT_2 * side_length;
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
    }

    fn render(&self, canvas: &sdl2::render::Canvas<sdl2::video::Window>)
    {
        self.particles
            .iter()
            .for_each(|p| p.render(canvas, sdl2::pixels::Color::RED));
    }
}
