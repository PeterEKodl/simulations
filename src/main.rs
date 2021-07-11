mod simulation;

use std::str::FromStr;



fn main() {
    println!("The following simulations are available:");
    
    let mut input_choice = String::default();

    let choice = loop
    {
        input_choice.clear();
        match std::io::stdin().read_line(&mut input_choice)
        {
            Ok(_) => {},
            Err(_) => {println!("Input must be a valid integer."); continue;}
        };
        break match usize::from_str(input_choice.trim())
        {
            Ok(value) => value,
            Err(_) => {println!("Input must be a valid integer."); continue;}
        };
    };
    println!("{}", choice);


    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let window = video_subsystem.window("Something", 100, 100).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
}
