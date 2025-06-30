use signalstashrs::sensor::{SensorData, Domain};
use prost::Message;
use std::fs::File;
use std::io::Write;

fn main() {
    let msg = SensorData {
        timestamp: 1723839123,
        datum: 42.5,
        domain: Domain::SoundPressureLevel as i32,
        device_id: b"testdevice".to_vec(),
    };
    let mut buf = Vec::new();
    msg.encode(&mut buf).expect("encode failed");
    let mut file = File::create("tests/http/sample_sensor_data.bin").expect("create file");
    file.write_all(&buf).expect("write failed");
    println!("Wrote sample_sensor_data.bin ({} bytes)", buf.len());
}
