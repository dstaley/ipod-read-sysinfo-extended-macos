use rusb::{request_type, Context, Direction, Recipient, RequestType, UsbContext};
use std::str;
use std::time::Duration;

fn main() {
    let context = Context::new().expect("couldn't instantiate libusb");
    let timeout = Duration::from_millis(1000);
    let devices = context.devices().expect("could not find any devices");

    'outer: for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == 0x05AC {
            let handle = match device.open() {
                Ok(h) => h,
                Err(_) => continue,
            };

            let mut data: [u8; 4096] = [0; 4096];
            let mut sysinfo: String = "".to_owned();

            for i in 0..0xffff {
                let bytes_read = match handle.read_control(
                    request_type(Direction::In, RequestType::Vendor, Recipient::Device),
                    0x40,
                    0x02,
                    i,
                    &mut data,
                    timeout,
                ) {
                    Ok(br) => br,
                    Err(_) => continue 'outer,
                };
                let r = match str::from_utf8(&data[..bytes_read]) {
                    Ok(s) => s,
                    Err(_) => continue 'outer
                };
                sysinfo.push_str(r);
                if bytes_read < data.len() {
                    break;
                }
            }

            println!("{}", sysinfo);
            break;
        }
    }
}
