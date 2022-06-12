pub mod devices;

pub mod smart_house {
    use crate::devices::{DeviceInfoProvider, Devices};
    use ::std::collections::{HashMap, HashSet};

    pub struct SmartHouse {
        pub title: String,
        /// Key is a room title, value is a map of named devices
        pub devices: HashMap<String, HashMap<Devices, HashSet<String>>>,
    }

    impl SmartHouse {
        pub fn create_report(&self, informer: &dyn DeviceInfoProvider) -> String {
            informer.get_report()
        }

        pub fn get_devices(&self) -> &HashMap<String, HashMap<Devices, HashSet<String>>> {
            &self.devices
        }

        pub fn get_room_devices(&self, room: &str) -> &HashMap<Devices, HashSet<String>> {
            &self.devices.get(room).unwrap()
        }

        pub fn get_rooms(&self) -> HashSet<&String> {
            let mut key_set = HashSet::with_capacity(self.devices.capacity());
            for x in &self.devices {
                key_set.insert(x.0);
            }
            key_set
        }

        pub fn check_device(&self, device: Devices, room: &str) -> &str {
            if !self.devices.contains_key(room) {
                "Room does not exists"
            } else if !self.devices.get(room).unwrap().contains_key(&device) {
                "Device does not exists in this room"
            } else {
                "Device is available in this room"
            }
        }
    }
}
