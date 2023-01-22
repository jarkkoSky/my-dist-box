use kanal::Receiver;
use wasapi::Direction;

use crate::audio_utils::init_audio_client;

pub fn playback(rx_play: Receiver<Vec<u8>>) {
    let (audio_client, h_event, blockalign, mut sample_queue) =
        init_audio_client(&Direction::Render);

    let render_client = audio_client.get_audiorenderclient().unwrap();

    audio_client.start_stream().unwrap();

    loop {
        let buffer_space_available = audio_client.get_available_space_in_frames().unwrap();

        while sample_queue.len() < (blockalign as usize * buffer_space_available as usize) {
            if let Ok(c) = rx_play.recv() {
                for element in c.iter() {
                    sample_queue.push_back(*element);
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
