mod common;
use common::{create_port, Port};

use bluerobotics_ping::{
    device::{Ping360, PingDevice},
    error::PingError,
};

#[tokio::main]
async fn main() -> Result<(), PingError> {
    println!("Parsing user provided values and creating port...");
    let port = create_port().await;

    println!("Creating your Ping 360 device");
    let ping360 = match port {
        Port::Serial(port) => Ping360::new(port),
        Port::Udp(port) => Ping360::new(port),
    };

    println!("Reading transducer data:");
    // Start auto-transmit in a 90 gradian cone
    // This will have no effect if the device is already in auto transmit mode
    // Starting this program while the device is already in auto transmit mode may cause auto_device_data request to hang until timeout
    // Stopping the program and immediatly rerunning should fix this
    ping360
        .auto_transmit(1, 1, 500, 20000, 700, 1000, 0, 90, 0, 0).await?;
    // Wait for and then print next packet forever
    loop {
        println!("Waiting for device data ...");
        let p = ping360.auto_device_data().await?;
        println!("mode: {}", p.mode);
        println!("gain_setting: {}", p.gain_setting);
        println!("angle: {}", p.angle);
        println!("transmit_duration: {}", p.transmit_duration);
        println!("sample_period: {}", p.sample_period);
        println!("transmit_frequency: {}", p.transmit_frequency);
        println!("number_of_samples: {}", p.number_of_samples);
        println!("data_length: {}", p.data_length);
        println!("data: {:?}", p.data);
    }
    unreachable!();
    // Creating futures to read different device Properties
    let (protocol_version_struct, device_information) =
        tokio::try_join!(ping360.protocol_version(), ping360.device_information())
            .expect("Failed to join results");

    let version = format!(
        "{}.{}.{}",
        protocol_version_struct.version_major,
        protocol_version_struct.version_minor,
        protocol_version_struct.version_patch
    );

    println!("Protocol version is: {version}");
    println!("Device information: {device_information:?}");

    Ok(())
}
