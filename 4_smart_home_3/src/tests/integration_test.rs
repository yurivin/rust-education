use smart_home_2::*;

#[test]
fn main_integration_test() {
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

    assert_eq!(house.title, String::from("Nice home"));
    assert_eq!(house.check_device(Devices::Thermometer, "kitchen"), String::from("Main"));

   // println!("Check field data {:#?}", borrower.device_a.item_type);
   // println!("Special report: {}", house.create_report(&owner));
   // println!("Special report: {}", house.create_report(&borrower));
}