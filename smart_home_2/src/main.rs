use ::std::collections::{HashMap, HashSet};
use std::fmt::Debug;
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
        house.check_device(Devices::Thermometer, "kitchen")
    );
    println!(
        "What about a Thermometer in the bedroom?\n   {}",
        house.check_device(Devices::Thermometer, "bedroom")
    );
    println!(
        "What about a Thermometer in the guestroom?\n   {}",
        house.check_device(Devices::Thermometer, "guestroom")
    );
    println!("Rooms list: {:#?}", house.get_rooms());
    println!("Device for a kitchen are : {:#?}", house.get_room_devices("kitchen"));

    let device_two = Device {
        title: String::from("Tongo"),
        status: String::from("Passive"),
        item_type: Devices::Thermometer
    };

    let owner = OwningDeviceInfoProvider {
        device: Device {
            title: String::from("Bongo"),
            status: String::from("Active"),
            item_type: Devices::Thermometer
        }
    };

    let borrower = BorrowingDeviceInfoProvider {
        device_a: &owner.device,
        device_b: &device_two
    };

    println!("Check field data {:#?}", borrower.device_a.item_type);
    println!("Special report: {}", house.create_report(&owner));
    println!("Special report: {}", house.create_report(&borrower));

}

struct SmartHouse {
    title: String,
    /// Key is a room title, value is a map of named devices
    devices: HashMap<String, HashMap<Devices, HashSet<String>>>,
}

struct OwningDeviceInfoProvider {
    device: Device,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    device_a: &'a Device,
    device_b: &'b Device,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_report(&self) -> String {
        self.device.get_report()
    }
}


impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_report(&self) -> String {
        let mut temp = String::from(self.device_a.get_report());
        temp.push_str("\n");
        temp.push_str(&self.device_b.get_report());
        temp
    }
}

impl DeviceInfoProvider for Device {
    fn get_report(&self) -> String {
        let mut temp = self.title.clone();
        temp.push_str(" ");
        temp.push_str(&self.status);
        temp
    }
}

impl SmartHouse {

    fn create_report(
        &self,
        informer: &dyn DeviceInfoProvider
    ) -> String {
        informer.get_report()
    }

    fn get_devices(&self) -> &HashMap<String, HashMap<Devices, HashSet<String>>> {
        &self.devices
    }

    fn get_room_devices(&self, room: &str) -> &HashMap<Devices, HashSet<String>> {
            &self.devices.get(room).unwrap()
    }

    fn get_rooms(&self) -> HashSet<&String> {
        let mut key_set = HashSet::with_capacity(self.devices.capacity());
        for x in &self.devices {
            key_set.insert(x.0);
        }
        key_set
    }

    fn check_device(&self, device: Devices, room: &str) -> &str {
        if !self.devices.contains_key(room) {
            "Room does not exists"
        } else if !self.devices.get(room).unwrap().contains_key(&device) {
            "Device does not exists in this room"
        } else {
            "Device is available in this room"
        }
    }
}

trait DeviceInfoProvider {
    fn get_report(&self) -> String;
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Devices {
    Rosette,
    Thermometer,
    Speaker,
}

struct Device {
    title: String,
    item_type: Devices,
    status: String
}
