use wasapi::Direction;

use crate::audio_utils::{init_audio_client, CHUNK_SIZE};

pub fn capture(tx_capt: std::sync::mpsc::SyncSender<Vec<u8>>) {
    let (audio_client, h_event, blockalign, mut sample_queue) =
        init_audio_client(&Direction::Capture);

    let render_client = audio_client.get_audiocaptureclient().unwrap();
    audio_client.start_stream().unwrap();

    loop {
        while sample_queue.len() > (blockalign as usize * CHUNK_SIZE as usize) {
            let mut chunk = vec![0u8; blockalign as usize * CHUNK_SIZE as usize];

            for element in chunk.iter_mut() {
                *element = sample_queue.pop_front().unwrap();
            }
            tx_capt.send(chunk).unwrap();
        }

        render_client
            .read_from_device_to_deque(blockalign as usize, &mut sample_queue)
            .unwrap();

        if h_event.wait_for_event(1000000).is_err() {
            audio_client.stop_stream().unwrap();
            break;
        }
    }
}
