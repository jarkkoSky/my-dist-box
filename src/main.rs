use std::io::{self};
use wasapi::{initialize_mta, Device, DeviceCollection, Direction};

fn select_device(direction: &Direction) -> Device {
    let devices = DeviceCollection::new(direction).unwrap();

    let number_of_devices = devices.get_nbr_devices().unwrap();

    match direction {
        Direction::Capture => {
            println!("Found {} input devices", number_of_devices)
        }
        Direction::Render => {
            println!("Found {} output devices", number_of_devices)
        }
    }

    for index in 0..number_of_devices {
        let device = devices.get_device_at_index(index).unwrap();

        println!("{}: {}", index, device.get_friendlyname().unwrap());
    }

    let mut user_select = String::new();

    println!("Choose device (enter index)");

    io::stdin()
        .read_line(&mut user_select)
        .expect("Failed to read line");

    devices
        .get_device_at_index(user_select.trim().parse().unwrap())
        .unwrap()
}

fn main() {
    initialize_mta().unwrap();

    let output_device = select_device(&Direction::Render);
    let input_device = select_device(&Direction::Capture);

    println!(
        "Input: {} \r\nOutput: {}",
        input_device.get_friendlyname().unwrap(),
        output_device.get_friendlyname().unwrap()
    );
}
