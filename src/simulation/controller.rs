use std::time::Duration;

// The time difference is constant so that the simulation is always deterministic.



pub trait Controller
{

    fn tick(&mut self, dt: &Duration);
    fn name(&self) -> &'static str;
    fn fetch_parameters_from_input(&mut self);
    

}
