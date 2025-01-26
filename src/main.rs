use clap::Parser;
mod device;
use device::Device;
use std::{thread, time, time::Duration};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Serial device to connect to
    #[arg(short, long)]
    device: String,
}

const SERIAL_PORT_BAUDRATE: u32 = 115200;

fn main() {
    env_logger::init();

    let args = Args::parse();

    let port = serialport::new(args.device, SERIAL_PORT_BAUDRATE)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open port");

    let mut device = Device {
        transport: Box::new(port),
    };

    /* Check link health */
    device.ping().unwrap();

    let wait = time::Duration::from_millis(100);
    for _ in 1..1000 {
        thread::sleep(wait);
        device.set_led(0, true).unwrap();
        thread::sleep(wait);
        device.set_led(0, false).unwrap();
        thread::sleep(wait);
    }
}
