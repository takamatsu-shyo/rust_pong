use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
mod hw;
mod communication;

fn main() {
    hw::hello_world();

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("data/boing_x.wav").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    let result_sound = stream_handle.play_raw(source.convert_samples());
    println!("play sound{:?}", result_sound);
    
    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));

    let result = communication::my_udp();
    println!("udp com {:?}", result);
}
