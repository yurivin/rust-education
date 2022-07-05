use std::str::Split;
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
    home: SmartHouse,
}

impl RequestHandler {
    pub fn new(home: SmartHouse) -> Self {
        Self { home }
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
        String::from("default get_rosette_state response")
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
        let chat = SmartHouse::default();
        let mut handler = RequestHandler::new(chat);

        let room_id = String::from("kitchen");
        let rossette_title = String::from("Left\r\n");
        let req_str = format!("rosette_state|||{}|||{}", room_id, rossette_title);
        let req = Request::new(&req_str);

        let fetched = handler.handle(req);

        assert_eq!(String::from("default get_rosette_state response"), fetched);
    }
}
