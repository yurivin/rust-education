use ::std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn main() {
    println!("Hello, Smart house 2!");

    let house = SmartHouse {
        title: String::from("Nice home"),
        devices: HashMap::from([
            (
                String::from("kitchen"),
                HashMap::from([
                    (
                        Device::Rosette,
                        HashSet::from([
                            String::from("Left"),
                            String::from("Right"),
                            String::from("Center"),
                        ]),
                    ),
                    (Device::Thermometer, HashSet::from([String::from("Main")])),
                ]),
            ),
            (
                String::from("bedroom"),
                HashMap::from([(
                    Device::Speaker,
                    HashSet::from([String::from("Left"), String::from("Right")]),
                )]),
            ),
        ]),
    };
    println!("My smart house name is {}", house.title);
    println!("Devices: {:#?}", house.get_devices());
    println!(
        "What about a Thermometer in the kitchen?\n   {}",
        house.device_report(Device::Thermometer, "kitchen")
    );
    println!(
        "What about a Thermometer in the bedroom?\n   {}",
        house.device_report(Device::Thermometer, "bedroom")
    );
    println!(
        "What about a Thermometer in the guestroom?\n   {}",
        house.device_report(Device::Thermometer, "guestroom")
    );
}

struct SmartHouse {
    title: String,
    /// Key is a room title, value is a map of named devices
    devices: HashMap<String, HashMap<Device, HashSet<String>>>,
}

impl SmartHouse {
    fn get_devices(&self) -> &HashMap<String, HashMap<Device, HashSet<String>>> {
        &self.devices
    }

    fn device_report(&self, device: Device, room: &str) -> &str {
        if !self.devices.contains_key(room) {
            "Room does not exists"
        } else if !self.devices.get(room).unwrap().contains_key(&device) {
            "Device does not exists in this room"
        } else {
            "Device is available in this room"
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Device {
    Rosette,
    Thermometer,
    Speaker,
}
