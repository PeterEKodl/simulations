mod simulation;

use sdl2::{event::Event, keyboard::Keycode, pixels};
use std::str::FromStr;
use std::time::{Duration, Instant};

const FPS: f32 = 100.0;
const SPF: f32 = 1.0 / FPS;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let mut simulations: Vec<Box<dyn simulation::Controller>> = vec![
        Box::new(simulation::GravityController::default()),
        Box::new(simulation::SandController::default()),
        Box::new(simulation::SoftbodyController::default()),
    ];

    println!("The following simulations are available:");
    simulations
        .iter()
        .enumerate()
        .for_each(|(i, x)| println!("{}: {}", i, x.name()));

    let mut input_choice = String::default();

    let choice = loop
    {
        input_choice.clear();
        if let Err(_) = std::io::stdin().read_line(&mut input_choice)
        {
            println!("Input error.");
            continue;
        };
        let choice = match usize::from_str(input_choice.trim())
        {
            Ok(value) => value,
            Err(_) =>
            {
                println!("Input must be a valid integer.");
                continue;
            }
        };

        if choice < simulations.len()
        {
            break choice;
        }
        else
        {
            println!("Input must be within range [0;{}]", simulations.len() - 1);
        }
    };
    let mut simulation = simulations.remove(choice);
    println!("You chose {}.", simulation.name());
    let bounds = simulation::SimulationBounds(1000.0, 600.0);
    simulation.fetch_parameters_from_input(&bounds);

    let sdl2_context = sdl2::init()?;
    let video_subsystem = sdl2_context.video()?;
    let window = video_subsystem
        .window(simulation.name(), bounds.0 as u32, bounds.1 as u32)
        .build()?;
    let mut canvas = window.into_canvas().build()?;
    let mut events = sdl2_context.event_pump()?;

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut now = Instant::now();
    let spf_duration = Duration::from_secs_f32(SPF);
    'main: loop
    {
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        simulation.render(&canvas);
        canvas.present();
        simulation.tick(&simulation::constants::DT, &bounds);
        if let Some(event) = events.poll_event()
        {
            match event
            {
                Event::Quit { .. } => break 'main,
                _ =>
                {}
            }
        }

        let new_now = Instant::now();

        std::thread::sleep(spf_duration.saturating_sub(new_now - now));
        now = new_now;
    }
    Ok(())
}
