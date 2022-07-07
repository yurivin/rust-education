use std::collections::{HashMap, HashSet};
use smart_house::devices::Devices;
use smart_house::devices::Devices::{Rosette, Speaker};
use smart_house::smart_house::SmartHouse;

#[test]
fn main_integration_test() {
    let mut house = SmartHouse::default();

    assert_eq!(house.title, String::from("Nice home"));
    assert!(house.check_device(Devices::Thermometer, "kitchen").is_ok());

    house.add_device(String::from("bedroom"), Rosette, String::from("New rosette"));

    assert!(house.get_room_devices("bedroom").unwrap().get(&Rosette).unwrap().contains("New rosette"));
    assert!(house.get_room_devices("bedroom").unwrap().get(&Speaker).unwrap().contains("Right"));

    house.add_device(String::from("sportroom"), Rosette, String::from("Sport rosette"));
    assert!(house.get_room_devices("sportroom").unwrap().get(&Rosette).unwrap().contains("Sport rosette"));

    house.remove_device("kitchen", &Devices::Rosette, "Left");

    assert!(!house.get_room_devices("kitchen").unwrap().get(&Rosette).unwrap().contains("Left"));

    house.remove_room("bedroom");

    assert!(!house.get_rooms().contains(&String::from("bedroom")));
}
