use clap::Parser;
mod device;
use device::Device;

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
        .open()
        .expect("Failed to open port");

    let mut device = Device {
        transport: Box::new(port),
    };

    device.ping().unwrap();
}
