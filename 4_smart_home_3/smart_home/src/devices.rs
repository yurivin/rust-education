use crate::devices::device_info_provider::ReportError;

pub trait DeviceInfoProvider {
    fn get_report(&self) -> Result<String, ReportError>;
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

impl DeviceInfoProvider for Device {
    fn get_report(&self) -> Result<String, ReportError> {
        if self.title.is_empty() {
            Err(ReportError::NoData)
        } else {
            let mut temp = self.title.clone();
            temp.push(' ');
            temp.push_str(&self.status);
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
