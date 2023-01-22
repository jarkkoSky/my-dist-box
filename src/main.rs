use kanal;
use std::thread;
use wasapi::initialize_mta;

pub mod audio_utils;
pub mod input;
pub mod output;

fn main() {
    initialize_mta().unwrap();

    let (tx_play, rx_play) = kanal::bounded::<Vec<u8>>(2);
    let (tx_capt, rx_capt) = kanal::bounded::<Vec<u8>>(2);

    // Playback
    let _handle = thread::Builder::new()
        .name("Player".to_string())
        .spawn(move || {
            output::playback(rx_play);
        });

    // Capture
    let _handle = thread::Builder::new()
        .name("Capture".to_string())
        .spawn(move || {
            input::capture(tx_capt);
        });

    loop {
        match rx_capt.recv() {
            Ok(chunk) => {
                tx_play.send(chunk).unwrap();
            }
            Err(err) => println!("Some error {}", err),
        }
    }
}
