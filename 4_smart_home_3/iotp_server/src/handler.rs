use smart_house::devices::Devices;
use smart_house::smart_house::SmartHouse;
use std::str::Split;
pub struct Request<'a>(Split<'a, &'a str>);

impl<'a> Request<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s.split("|||"))
    }

    pub fn next(&mut self) -> &'a str {
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
        let command = request.next();

        let room_id = request.next();
        if room_id.is_empty() {
            return "No room id received".into();
        }

        let rosette = request.next();
        if rosette.is_empty() {
            return "No device name received".into();
        }
        match command {
            "state" => self.get_rosette_state(room_id, rosette),
            "power" => self.get_rosette_power(room_id, rosette),
            _ => "Bad command".into(),
        }
    }

    fn get_rosette_state(&self, room_id: &str, rosette: &str) -> String {
        let state = Devices::get_state(rosette, room_id, &self.house, Devices::Rosette);
        match state {
            Some(status) => status.to_string(),
            None => "Unknown device".to_string(),
        }
    }

    fn get_rosette_power(&self, room_id: &str, rosette: &str) -> String {
        let power = Devices::power(
            rosette.trim(),
            room_id.trim(),
            &self.house,
            Devices::Rosette,
        )
        .unwrap_or(0 as f32);
        power.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::handler::{Request, RequestHandler};
    use smart_house::smart_house::SmartHouse;

    #[test]
    fn test_commands() {
        let house = SmartHouse::default();
        let mut handler = RequestHandler::new(house);

        let room_id = String::from("kitchen");
        let rossette_title = String::from("Left\r\n");
        let req_str = format!("power|||{}|||{}", room_id, rossette_title);
        let req = Request::new(&req_str);

        let fetched = handler.handle(req);

        assert_eq!(4.to_string(), fetched);

        // Test case 2
        let room_id = String::from("pitchen");
        let rosette_title = String::from("Left\r\n");
        let req_str = format!("power|||{}|||{}", room_id, rosette_title);
        let req = Request::new(&req_str);

        let fetched = handler.handle(req);

        assert_eq!(4.to_string(), fetched);

        //Test case 3
        let room_id = String::from("kitchen");
        let rosette_title = String::from("Left\r\n");
        let req_str = format!("state|||{}|||{}", room_id, rosette_title);
        let req = Request::new(&req_str);

        let fetched = handler.handle(req);

        assert_eq!("Active", fetched);

        //Test case 4
        let room_id = String::from("kitchen");
        let rossette_title = String::from("Beft\r\n");
        let req_str = format!("state|||{}|||{}", room_id, rossette_title);
        let req = Request::new(&req_str);

        let fetched = handler.handle(req);

        assert_eq!("Unknown device", fetched);
    }
}
