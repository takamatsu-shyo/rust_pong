use std::fs::File;
use std::io;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};


mod hw;
mod communication;

fn main() -> Result<(), io::Error>  {
    //hw::hello_world();

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                ].as_ref()
            )
            .split(f.size());
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default()
            .title("Block 2")
            .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    });

    

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

    // UDP
    //let result = communication::my_udp();
    //println!("udp com {:?}", result);

    Ok(())
}
