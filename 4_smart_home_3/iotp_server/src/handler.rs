use smart_house::devices::device_info_provider::OwningDeviceInfoProvider;
use smart_house::devices::{Device, Devices};
use smart_house::smart_house::SmartHouse;
use std::ops::Add;
use std::str::{FromStr, Split};

pub struct Request<'a>(Split<'a, &'a str>);

impl<'a> Request<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s.split("|||"))
    }

    pub fn next_request(&mut self) -> &'a str {
        self.0.next().unwrap_or("")
    }
}

pub struct RequestHandler {
    house: SmartHouse,
}

impl RequestHandler {
    pub fn new(home: SmartHouse) -> Self {
        Self { house: home }
    }

    pub fn handle(&mut self, mut request: Request) -> String {
        let command = request.next_request();

        let room_id = request.next_request();
        if room_id.is_empty() {
            return "No room id received".into();
        }

        let device_type = request.next_request();
        if device_type.is_empty() {
            return "No device type received".into();
        }

        let device = request.next_request();
        if device.is_empty() {
            return "No device name received".into();
        }
        match command {
            "state" => self.get_state(room_id, device_type, device),
            "power" => self.get_power(room_id, device_type, device),
            "switch" => self.switch(room_id.trim(), device_type.trim(), device.trim()),
            _ => "Bad command".into(),
        }
    }

    fn switch(&mut self, room_id: &str, device_type: &str, rosette: &str) -> String {
        let store_id = room_id.to_owned().add(device_type).add(rosette);
        if self.house.store.get(&store_id).is_some() {
            let device_old = &self.house.store.get(&store_id).unwrap().device.clone();
            self.house.store.insert(
                store_id.clone(),
                OwningDeviceInfoProvider {
                    device: Device {
                        title: device_old.title.clone(),
                        item_type: device_old.item_type.clone(),
                        status: device_old.status.opposite(),
                        data: u16::default()
                    },
                },
            );
            "Switched to ".to_owned().add(
                &self
                    .house
                    .store
                    .get(&store_id)
                    .unwrap()
                    .device
                    .status
                    .to_string(),
            )
        } else {
            String::from("Unknown device")
        }
    }

    fn get_state(&self, room_id: &str, device_type: &str, rosette: &str) -> String {
        let unknown = String::from("Unknown device");
        let device_type = Devices::from_str(device_type).expect(&unknown);
        let state = Devices::get_state(rosette, room_id, &self.house, device_type);
        match state {
            Some(status) => status.to_string(),
            None => unknown,
        }
    }

    fn get_power(&self, room_id: &str, device_type: &str, rosette: &str) -> String {
        let unknown = String::from("Unknown device");
        let device_type = Devices::from_str(device_type).expect(&unknown);
        let power = Devices::power(rosette.trim(), room_id.trim(), &self.house, device_type)
            .unwrap_or(0 as f32);
        power.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::handler::{Request, RequestHandler};
    use smart_house::devices::Devices;
    use smart_house::smart_house::SmartHouse;

    #[test]
    fn test_commands() {
        let house = SmartHouse::default();
        let mut handler = RequestHandler::new(house);

        let room_id = String::from("kitchen");
        let device_type = Devices::Rosette.to_string();
        let rossette_title = String::from("Left\r\n");
        let req_str = format!("power|||{}|||{}|||{}", room_id, device_type, rossette_title);
        let req = Request::new(&req_str);

        let fetched = handler.handle(req);

        assert_eq!(4.to_string(), fetched);

        // Test case 2
        let room_id = String::from("pitchen");
        let device_type = Devices::Rosette.to_string();
        let rosette_title = String::from("Left\r\n");
        let req_str = format!("power|||{}|||{}|||{}", room_id, device_type, rosette_title);
        let req = Request::new(&req_str);

        let fetched = handler.handle(req);

        assert_eq!(4.to_string(), fetched);

        //Test case 3
        let room_id = String::from("kitchen");
        let device_type = Devices::Rosette.to_string();
        let rosette_title = String::from("Left\r\n");
        let req_str = format!("state|||{}|||{}|||{}", room_id, device_type, rosette_title);
        let req = Request::new(&req_str);

        let fetched = handler.handle(req);

        assert_eq!("Active", fetched);

        //Test case 4
        let room_id = String::from("kitchen");
        let device_type = Devices::Rosette.to_string();
        let rosette_title = String::from("Beft\r\n");
        let req_str = format!("state|||{}|||{}|||{}", room_id, device_type, rosette_title);
        let req = Request::new(&req_str);

        let fetched = handler.handle(req);

        assert_eq!("Unknown device", fetched);

        //Test case 5
        let room_id = String::from("kitchen");
        let device_type = Devices::Rosette.to_string();
        let rosette_title = String::from("Left\r\n");
        let req_str = format!("switch|||{}|||{}|||{}", room_id, device_type, rosette_title);
        let req = Request::new(&req_str);
        println!(
            "kitchenRosetteLeft state is {}",
            handler
                .house
                .store
                .get("kitchenRosetteLeft")
                .unwrap()
                .device
                .status
                .to_string()
        );
        assert_eq!(
            "Available",
            handler
                .house
                .store
                .get("kitchenRosetteLeft")
                .unwrap()
                .device
                .status
                .to_string()
        );

        let fetched = handler.handle(req);

        assert_eq!("Switched to Active", fetched);
        assert_eq!(
            "Active",
            handler
                .house
                .store
                .get("kitchenRosetteLeft")
                .unwrap()
                .device
                .status
                .to_string()
        )
    }
}
