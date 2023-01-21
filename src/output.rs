use std::sync::mpsc;

use wasapi::Direction;

use crate::audio_utils::init_audio_client;

pub fn playback(rx_play: std::sync::mpsc::Receiver<Vec<u8>>) {
    let (audio_client, h_event, blockalign, mut sample_queue) =
        init_audio_client(&Direction::Render);

    let render_client = audio_client.get_audiorenderclient().unwrap();

    audio_client.start_stream().unwrap();

    loop {
        let buffer_space_available = audio_client.get_available_space_in_frames().unwrap();

        while sample_queue.len() < (blockalign as usize * buffer_space_available as usize) {
            match rx_play.try_recv() {
                Ok(chunk) => {
                    for element in chunk.iter() {
                        sample_queue.push_back(*element);
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {
                    for _ in 0..((blockalign as usize * buffer_space_available as usize)
                        - sample_queue.len())
                    {
                        sample_queue.push_back(0);
                    }
                }
                Err(_) => {
                    println!("error");

                    break;
                }
            }
        }

        render_client
            .write_to_device_from_deque(
                buffer_space_available as usize,
                blockalign as usize,
                &mut sample_queue,
                None,
            )
            .unwrap();

        if h_event.wait_for_event(100000).is_err() {
            println!("error, stopping playback");
            audio_client.stop_stream().unwrap();
            break;
        }
    }
}
