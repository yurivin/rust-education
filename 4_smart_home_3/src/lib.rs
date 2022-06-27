pub mod devices;

pub mod smart_house {
    use crate::devices::device_info_provider::ReportError;
    use crate::devices::{DeviceInfoProvider, Devices};
    use ::std::collections::{HashMap, HashSet};

    pub struct SmartHouse {
        pub title: String,
        pub purpose: String,
        /// Key is a room title, value is a map of named devices
        pub devices: HashMap<String, HashMap<Devices, HashSet<String>>>,
    }

    #[derive(Debug)]
    pub enum SmartHouseError {
        RoomNotExists,
        DeviceNotExistsInThisRoom,
    }

    impl SmartHouse {
        pub fn remove_room(&mut self, room: &String) -> () {
            if self.devices.contains_key(room) {
                self.devices.remove(room);
            }
        }

 /*       pub fn remove_device(&mut self, room:String, device: Devices, device_title: &String) {

        }*/

        pub fn add_device(&mut self, room: String, device: Devices, device_title: String) -> () {
            if self.devices.contains_key(&room) {
                let room_map = self.devices.get_mut(&room).unwrap();
                if room_map.contains_key(&device) {
                    room_map
                        .get_mut(&device)
                        .unwrap()
                        .extend(HashSet::from([device_title]).into_iter());
                } else {
                    room_map.insert(device, HashSet::from([device_title]));
                }
            } else {
                self.devices.insert(
                    room,
                    HashMap::from([
                        (device, HashSet::from([device_title])),
                        (Devices::Thermometer, HashSet::from([String::from("Main")])),
                    ]),
                );
            }
        }

        pub fn create_report(
            &self,
            informer: &dyn DeviceInfoProvider,
        ) -> Result<String, ReportError> {
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

        pub fn check_device(&self, device: Devices, room: &str) -> Result<bool, SmartHouseError> {
            if !self.devices.contains_key(room) {
                Result::Err(SmartHouseError::RoomNotExists)
            } else if !self.devices.get(room).unwrap().contains_key(&device) {
                Result::Err(SmartHouseError::DeviceNotExistsInThisRoom)
            } else {
                Result::Ok(true)
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
            .get_room_devices("bedroom")
            .unwrap()
            .get(&Devices::Speaker)
            .unwrap()
            .contains("Left"));
        assert!(house
            .get_room_devices("bedroom")
            .unwrap()
            .get(&Devices::Speaker)
            .unwrap()
            .contains("Right"));
        assert_eq!(house.get_room_devices("bedroom").unwrap().len(), 1);
    }
}
