use ::std::collections::{HashMap, HashSet};
use smart_home_2::devices::device_info_provider::{
    BorrowingDeviceInfoProvider, OwningDeviceInfoProvider,
};
use smart_home_2::devices::*;
use smart_home_2::smart_house::SmartHouse;

fn main() {
    println!("Hello, Smart house 2!");

    let house = SmartHouse {
        title: String::from("Nice home"),
        purpose: String::from("For rent"),
        devices: HashMap::from([
            (
                String::from("kitchen"),
                HashMap::from([
                    (
                        Devices::Rosette,
                        HashSet::from([
                            String::from("Left"),
                            String::from("Right"),
                            String::from("Center"),
                        ]),
                    ),
                    (Devices::Thermometer, HashSet::from([String::from("Main")])),
                ]),
            ),
            (
                String::from("bedroom"),
                HashMap::from([(
                    Devices::Speaker,
                    HashSet::from([String::from("Left"), String::from("Right")]),
                )]),
            ),
        ]),
    };
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
        status: String::from("Passive"),
        item_type: Devices::Thermometer,
    };

    let owner = OwningDeviceInfoProvider {
        device: Device {
            title: String::from("Bongo"),
            status: String::from("Active"),
            item_type: Devices::Thermometer,
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
