use crate::devices::device_info_provider::BorrowingDeviceInfoProvider;

pub trait DeviceInfoProvider {
    fn get_report(&self) -> String;
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Devices {
    Rosette,
    Thermometer,
    Speaker,
}

pub struct Device {
    pub title: String,
    pub item_type: Devices,
    pub status: String,
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

pub mod device_info_provider {
    use crate::devices::{Device, DeviceInfoProvider};
    pub struct OwningDeviceInfoProvider {
        pub device: Device,
    }
    pub struct BorrowingDeviceInfoProvider<'a, 'b> {
        pub device_a: &'a Device,
        pub device_b: &'b Device,
    }

    impl DeviceInfoProvider for OwningDeviceInfoProvider {
        fn get_report(&self) -> String {
            self.device.get_report()
        }
    }
}