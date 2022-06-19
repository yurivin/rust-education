use smart_home_2::devices::Devices;
use smart_home_2::smart_house::SmartHouse;
use std::collections::{HashMap, HashSet};

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
    assert!(house.check_device(Devices::Thermometer, "kitchen").is_ok());
}
