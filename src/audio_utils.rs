use std::collections::VecDeque;

use wasapi::{
    AudioClient, Device, DeviceCollection, Direction, Handle, SampleType, ShareMode, WaveFormat,
};

pub const CHUNK_SIZE: u8 = 128;
pub const SAMPLE_QUEUE_CAPACITY: usize = 2048;

fn sample_queue_init() -> VecDeque<u8> {
    VecDeque::with_capacity(SAMPLE_QUEUE_CAPACITY)
}

fn select_device(direction: &Direction) -> Device {
    let devices = DeviceCollection::new(direction).unwrap();

    match direction {
        Direction::Capture => {
            let device = devices.get_device_at_index(0).unwrap();

            println!("Input device: {}", device.get_friendlyname().unwrap());
            device
        }
        Direction::Render => {
            let device = devices.get_device_at_index(2).unwrap();

            println!("Output device: {}", device.get_friendlyname().unwrap());
            device
        }
    }
}

pub fn init_audio_client(direction: &Direction) -> (AudioClient, Handle, u32, VecDeque<u8>) {
    let device = select_device(&direction);

    let mut audio_client = device.get_iaudioclient().unwrap();
    let desired_format = WaveFormat::new(32, 32, &SampleType::Float, 44100, 2);

    let blockalign = desired_format.get_blockalign();
    let (_, min_time) = audio_client.get_periods().unwrap();

    audio_client
        .initialize_client(
            &desired_format,
            min_time as i64,
            direction,
            &ShareMode::Shared,
            true,
        )
        .unwrap();

    let h_event = audio_client.set_get_eventhandle().unwrap();

    (audio_client, h_event, blockalign, sample_queue_init())
}
