use std::collections::{HashMap, HashSet};
use smart_home_2::devices::Devices;
use smart_home_2::smart_house::SmartHouse;

#[test]
fn main_integration_test() {
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

    assert_eq!(house.title, String::from("Nice home"));
    assert_eq!(house.check_device(Devices::Thermometer, "kitchen"), "Device is available in this room");
}