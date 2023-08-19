#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;
use uefi::proto::console::serial::Serial;
use uefi::proto::device_path::text::DevicePathFromText;
use uefi::table::boot::SearchType;
use uefi::Identify;

fn test_serial(boot_services: &BootServices) {
    let device_path_from_text_handle = *boot_services
        .locate_handle_buffer(SearchType::ByProtocol(&DevicePathFromText::GUID))
        .expect("DevicePathFromText is missing")
        .first()
        .expect("DevicePathFromText is missing");

    let device_path_from_text = boot_services
        .open_protocol_exclusive::<DevicePathFromText>(device_path_from_text_handle)
        .expect("DevicePathFromText unavailable");

    let serial_handle = boot_services
        .locate_device_path::<Serial>(
            &mut device_path_from_text
                .convert_text_to_device_path(cstr16!(
                    "PciRoot(0x0)/Pci(0x1,0x0)/Serial(0x1)/Uart(115200,8,N,1)"
                ))
                .expect("Device path not valid"),
        )
        .expect("Device path not found");

    let mut serial_protocol = boot_services
        .open_protocol_exclusive::<Serial>(serial_handle)
        .expect("Serial device unavailable");

    match serial_protocol.write("Hello world!".as_bytes()) {
        Err(count) => {
            info!("Error in serial.write(): only {count} bytes written");
        }
        Ok(_) => {
            info!("serial.write() Ok");
        }
    };
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let boot_services = system_table.boot_services();

    system_table.boot_services().stall(10_000_000);
    test_serial(boot_services);
    system_table.boot_services().stall(10_000_000);

    Status::SUCCESS
}
