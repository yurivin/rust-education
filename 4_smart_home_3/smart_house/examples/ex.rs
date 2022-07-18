use smart_house::devices::device_info_provider::{
    BorrowingDeviceInfoProvider, OwningDeviceInfoProvider,
};
use std::sync::atomic::AtomicU16;
use std::sync::Arc;

use smart_house::devices::{Device, DeviceState, Devices};
use smart_house::smart_house::SmartHouse;

fn main() {
    println!("Hello, Smart house 2!");

    let house = SmartHouse::default();
    println!("My smart house name is {}", house.title);
    println!("Devices: {:#?}", house.get_devices());
    println!(
        "What about a Thermometer in the kitchen?\n   {}",
        house.check_device(Devices::Thermometer, "kitchen").unwrap()
    );
    println!(
        "What about a Thermometer in the bedroom?\n   {}",
        house.check_device(Devices::Thermometer, "bedroom").unwrap()
    );
    println!(
        "What about a Thermometer in the guestroom?\n   {}",
        house
            .check_device(Devices::Thermometer, "guestroom")
            .unwrap()
    );
    println!("Rooms list: {:#?}", house.get_rooms());
    println!(
        "Device for a kitchen are : {:#?}",
        house.get_room_devices("kitchen")
    );

    let device_two = Device {
        title: String::from("Tongo"),
        status: DeviceState::Available,
        item_type: Devices::Thermometer,
        data: Arc::new(AtomicU16::default()),
    };

    let owner = OwningDeviceInfoProvider {
        device: Device {
            title: String::from("Bongo"),
            status: DeviceState::Active,
            item_type: Devices::Thermometer,
            data: Arc::new(AtomicU16::default()),
        },
    };

    let borrower = BorrowingDeviceInfoProvider {
        device_a: &owner.device,
        device_b: &device_two,
    };

    println!("Check field data {:#?}", borrower.device_a.item_type);
    println!("Special report: {}", house.create_report(&owner).unwrap());
    println!(
        "Special report: {}",
        house.create_report(&borrower).unwrap()
    );
}
