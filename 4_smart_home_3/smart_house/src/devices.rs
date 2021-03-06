use crate::devices::device_info_provider::ReportError;
use crate::smart_house::{SmartHouse, SmartHouseError};
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use std::{fmt, net, thread};

pub trait DeviceInfoProvider {
    fn get_report(&self) -> Result<String, ReportError>;
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Devices {
    Rosette,
    Thermometer,
    Speaker,
}

impl fmt::Display for Devices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for Devices {
    type Err = SmartHouseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Rosette" => Ok(Devices::Rosette),
            "Thermometer" => Ok(Devices::Thermometer),
            "Speaker" => Ok(Devices::Speaker),
            _ => Err(SmartHouseError::UnknownDeviceType),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DeviceState {
    Active,
    Available,
}

impl DeviceState {
    pub fn opposite(&self) -> DeviceState {
        match self {
            DeviceState::Active => DeviceState::Available,
            DeviceState::Available => DeviceState::Active,
        }
    }
}

impl fmt::Display for DeviceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Devices {
    pub fn temperature(room_id: &str, device_title: &str, house: &SmartHouse) -> String {
        let mut key = device_title.to_owned();
        key.push_str("Thermometer");
        key.push_str(room_id);
        println!("The key is {}", key);
        let data = house.store.get(&key).unwrap().device.data.clone();
        data.fetch_or(0, Ordering::SeqCst).to_string()
    }
    pub fn power(
        device_title: &str,
        room_id: &str,
        house: &SmartHouse,
        device_type: Devices,
    ) -> Result<f32, SmartHouseError> {
        println!(
            "Showing power for house {} room {} device {} of type {:?}",
            house.title, room_id, device_title, device_type
        );
        Ok(device_title.len() as f32)
    }

    pub fn get_state(
        device_title: &str,
        room_id: &str,
        house: &SmartHouse,
        device_type: Devices,
    ) -> Option<DeviceState> {
        println!(
            "Showing state for house {} room {} device {} of type {:?}",
            house.title, room_id, device_title, device_type
        );
        let room_devices_option = house.get_room_devices(room_id);
        println!(
            "There are devices in the room: {}",
            room_devices_option.is_some()
        );
        println!(
            "There are devices of type {:?} in the room: {:?}",
            device_type,
            room_devices_option.unwrap().get(&device_type).is_some()
        );
        println!(
            "list of devices of type {:?} in the room: {:?}",
            device_type,
            room_devices_option.unwrap().get(&device_type).unwrap()
        );
        let typed_devices = room_devices_option.unwrap().get(&device_type).unwrap();
        println!(
            "Room contains device with name {}: {}",
            device_title,
            typed_devices.contains(device_title)
        );

        if let Some(..) = room_devices_option {
            if room_devices_option.unwrap().get(&device_type).is_some() {
                let availability = room_devices_option
                    .unwrap()
                    .get(&device_type)
                    .unwrap()
                    .get(device_title.trim());
                println!("Availability: {}", availability.is_some());
                match availability {
                    Some(_) => return Some(DeviceState::Active),
                    None => return None,
                }
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct Device {
    pub title: String,
    pub item_type: Devices,
    pub status: DeviceState,
    pub data: Arc<AtomicU16>,
}

impl Device {
    pub fn listen(&self, address: String) {
        let arc_data = self.data.clone();

        thread::spawn(move || {
            let mut buffer: [u8; 2] = [0; 2];
            let socket = net::UdpSocket::bind(address).expect("failed to bind host socket");

            loop {
                let (number_of_bytes, src_address) =
                    socket.recv_from(&mut buffer).expect("no data received");

                println!("{:?}", number_of_bytes);
                println!("{:?}", src_address);

                arc_data.store(u16::from_be_bytes(buffer), Ordering::SeqCst);
            }
        });
    }
}

impl DeviceInfoProvider for Device {
    fn get_report(&self) -> Result<String, ReportError> {
        if self.title.is_empty() {
            Err(ReportError::NoData)
        } else {
            let mut temp = self.title.clone();
            temp.push(' ');
            temp.push_str(&self.status.to_string());
            Ok(temp)
        }
    }
}

pub mod device_info_provider {
    use crate::devices::device_info_provider::ReportError::NoData;
    use crate::devices::{Device, DeviceInfoProvider};
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    pub enum ReportError {
        NoData,
    }

    impl Display for ReportError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                NoData => write!(f, "No data"),
            }
        }
    }

    impl Error for ReportError {}

    pub struct OwningDeviceInfoProvider {
        pub device: Device,
    }
    pub struct BorrowingDeviceInfoProvider<'a, 'b> {
        pub device_a: &'a Device,
        pub device_b: &'b Device,
    }

    impl DeviceInfoProvider for OwningDeviceInfoProvider {
        fn get_report(&self) -> Result<String, ReportError> {
            self.device.get_report()
        }
    }

    impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
        fn get_report(&self) -> Result<String, ReportError> {
            let temp = self.device_a.get_report();
            let mut report = String::new();
            if let Ok(..) = temp {
                report.push_str(&*temp.unwrap());
                report.push('\n');
            }
            if self.device_b.get_report().is_ok() {
                report.push_str(&*self.device_b.get_report().unwrap());
            }
            if report.is_empty() {
                Err(NoData)
            } else {
                Ok(report)
            }
        }
    }
}
