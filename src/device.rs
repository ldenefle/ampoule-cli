use prost::Message;
use std::io::{Read, Write};
use std::time::{Duration, Instant};
pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/ampoule.rs"));
}

use anyhow::{anyhow, Result};
use log::trace;

pub trait Transport: Read + Write + Send {}
impl<T: Read + Write + Send> Transport for T {}

pub struct Device {
    pub transport: Box<dyn Transport>,
}

impl Device {
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<()> {
        const TIMEOUT_S: u64 = 2;
        let mut total_read: usize = 0;

        let start_time = Instant::now(); // Get the current time

        while start_time.elapsed() < Duration::from_secs(TIMEOUT_S) {
            let read = self.transport.read(&mut buf[total_read..total_read + 1])?;

            total_read += read;

            if total_read == buf.len() {
                return Ok(());
            }
        }
        Err(anyhow!("Not enough bytes to read"))
    }

    fn send_command(&mut self, command: protos::Command) -> Result<protos::Response> {
        let mut bytes = vec![];

        command.encode(&mut bytes)?;

        let size = bytes.len() as u16;

        let mut payload = vec![];
        payload.append(&mut Vec::from(size.to_be_bytes()));
        payload.append(&mut bytes);

        trace!("Transmitting {:02X?}", payload);

        let written = self.transport.write(&payload)?;

        if written != payload.len() {
            return Err(anyhow!("Could not write payload"));
        }

        let mut payload_size = [0; 2];
        self.read_bytes(&mut payload_size)?;

        let payload_size = ((payload_size[0] as u16) << 8) | payload_size[1] as u16;

        let mut payload: Vec<u8> = vec![0; payload_size.into()];

        self.read_bytes(&mut payload)?;

        trace!("Receiving {:02X?}", payload);

        let response = protos::Response::decode(&payload as &[u8])?;

        if !response.success {
            Err(anyhow!("Command failed"))
        } else {
            Ok(response)
        }
    }

    pub fn set_led(&mut self, index: u32, on: bool) -> Result<()> {
        let color: i32 = if on {
            protos::led::Color::White.into()
        } else {
            protos::led::Color::Off.into()
        };

        let led = protos::Led { color, index };

        let command = protos::Command {
            opcode: protos::Opcode::SetLed as i32,
            operation: Some(protos::command::Operation::Led(led)),
        };

        let response = self.send_command(command)?;

        if response.opcode != protos::Opcode::SetLed.into() {
            Err(anyhow!("Got wrong opcode as response"))
        } else {
            Ok(())
        }
    }

    pub fn ping(&mut self) -> Result<()> {
        let command = protos::Command {
            opcode: protos::Opcode::Ping as i32,
            operation: None,
        };

        let response = self.send_command(command)?;

        if response.opcode != protos::Opcode::Pong.into() {
            Err(anyhow!("Got wrong opcode as response"))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use mockall::predicate::eq;

    mock! {
        Transport {}
        impl Read for Transport {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
        }
        impl Write for Transport {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
            fn flush(&mut self) -> std::io::Result<()>;
        }
    }

    fn generate_proto<T: Message>(message: T) -> Vec<u8> {
        let mut ret = vec![];
        let mut payload = message.encode_to_vec();

        ret.append(&mut (payload.len() as u16).to_be_bytes().to_vec());
        ret.append(&mut payload);
        ret
    }

    #[test]
    fn test_ping_gets_ponged() {
        let mut transport = MockTransport::new();

        let command = generate_proto(protos::Command {
            opcode: protos::Opcode::Ping.into(),
        });

        let mut response = generate_proto(protos::Response {
            opcode: protos::Opcode::Pong.into(),
        });

        transport
            .expect_write()
            .with(eq(command))
            .returning(|buf| Ok(buf.len()));

        transport.expect_read().returning(move |buf| {
            let len = buf.len();
            buf.copy_from_slice(&response[..len]);
            response.drain(..len);
            Ok(len)
        });

        let mut steelix = Device {
            transport: Box::new(transport),
        };

        steelix.ping().unwrap();
    }
}
