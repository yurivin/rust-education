use iotp::client::{IotpClient, RequestResult};
use iotp::error::ConnectResult;
use std::net::ToSocketAddrs;

pub struct HouseClient {
    iotp: IotpClient,
}

impl HouseClient {
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> ConnectResult<Self> {
        let iotpc = IotpClient::connect(addr)?;
        Ok(Self { iotp: iotpc })
    }

    pub fn state(&mut self, room_id: &str, device_type: &str, device: &str) -> RequestResult {
        let request = format!("state|||{}|||{}|||{}", room_id, device_type, device);
        self.iotp.send_request(request)
    }

    pub fn power(&mut self, room_id: &str, device_type: &str, device: &str) -> RequestResult {
        let request = format!("power|||{}|||{}|||{}", room_id, device_type, device);
        self.iotp.send_request(request)
    }

    pub fn switch(&mut self, room_id: &str, device_type: &str, device: &str) -> RequestResult {
        let request = format!("switch|||{}|||{}|||{}", room_id, device_type, device);
        self.iotp.send_request(request)
    }
}

#[cfg(test)]
mod tests {
    use crate::HouseClient;
    use iotp_server::runner;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn it_works() {
        let address = "127.0.0.1:55331".to_string();
        let addr_copy = address.clone();

        thread::spawn(move || {
            if runner::run(addr_copy).is_err() {
                panic!("Error running server")
            }
        });

        let wclient = HouseClient::new(address.clone());
        thread::sleep(Duration::from_millis(2000));
        let client = wclient.expect("Unsuccessful connection");

        //Test case 1 - state
        // let client = wclient.expect("Unsuccessful connect");
    }
}
