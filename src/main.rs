use std::io;

mod hello_world;
mod communication;
mod gui;
mod sound;
mod engine;

fn main() -> Result<(), io::Error>  {

//    hello_world::hello_world();
//
//    let result = gui::gui();
//    println!("GUI {:?}", result);
//
//    let result = sound::sound();
//    println!("Sound {:?}", result);
//   
//    let result = communication::my_udp();
//    println!("udp com {:?}", result);

    engine::run();

    Ok(())
}
