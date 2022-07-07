use std::str::Split;
use smart_house::devices::Devices;
use smart_house::smart_house::SmartHouse;

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
        Self {house: home}
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
            "rosette_state" => self.get_rosette_state(room_id, rosette),
            "rosette_power" => self.get_rosette_power(room_id, rosette),
            _ => "Bad command".into(),
        }
    }

    fn get_rosette_state(&self, room_id: &str, rosette: &str) -> String {
        let power = Devices::current_power(rosette, room_id, &self.house).unwrap_or(0 as f32);
        power.to_string()
    }

    fn get_rosette_power(&self, room_id: &str, rosette: &str) -> String {
        String::from("default get_rosette_power response")
    }

}

#[cfg(test)]
mod tests {
    use smart_house::smart_house::SmartHouse;
    use crate::handler::{Request, RequestHandler};

    #[test]
    fn test_commands() {
        let house = SmartHouse::default();
        let mut handler = RequestHandler::new(house);

        let room_id = String::from("kitchen");
        let rossette_title = String::from("Left\r\n");
        let req_str = format!("rosette_state|||{}|||{}", room_id, rossette_title);
        let req = Request::new(&req_str);

        let fetched = handler.handle(req);

        assert_eq!(6.to_string(), fetched);

        // Test case 2
        let room_id = String::from("pitchen");
        let rossette_title = String::from("Left\r\n");
        let req_str = format!("rosette_state|||{}|||{}", room_id, rossette_title);
        let req = Request::new(&req_str);

        let fetched = handler.handle(req);

        assert_eq!(6.to_string(), fetched);
    }
}
