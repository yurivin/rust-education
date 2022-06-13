pub mod devices;

pub mod smart_house {
    use crate::devices::{DeviceInfoProvider, Devices};
    use ::std::collections::{HashMap, HashSet};

    pub struct SmartHouse {
        pub title: String,
        pub purpose: String,
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

        pub fn get_room_devices(&self, room: &str) -> Option<&HashMap<Devices, HashSet<String>>> {
            self.devices.get(room)
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

#[cfg(test)]
mod test {
    use crate::devices::Devices;
    use crate::smart_house::SmartHouse;
    use ::std::collections::{HashMap, HashSet};
    #[test]
    fn test_smart_house_properties() {
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

        assert_eq!(house.purpose, "For rent");
        assert_eq!(house.title, "Nice home");
        assert!(house
            .get_room_devices("bedroom").unwrap()
            .get(&Devices::Speaker)
            .unwrap()
            .contains("Left"));
        assert!(house
            .get_room_devices("bedroom").unwrap()
            .get(&Devices::Speaker)
            .unwrap()
            .contains("Right"));
        assert_eq!(house.get_room_devices("bedroom").unwrap().len(), 1);
    }
}
