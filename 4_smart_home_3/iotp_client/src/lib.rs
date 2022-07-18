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

    pub fn get_temperature(&mut self, room_id: &str, device_type: &str, device: &str) -> RequestResult {
        let request = format!("temp|||{}|||{}|||{}", room_id, device_type, device);
        self.iotp.send_request(request)
    }
}

#[cfg(test)]
mod tests {
    use crate::HouseClient;
    use iotp_server::runner;
    use std::{net, thread};
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
        let mut client = wclient.expect("Unsuccessful connection");

        //Test case 1 - state
        let request_result = client.state("kitchen", "Rosette", "Left");
        assert!(request_result.is_ok());
        let result = request_result.unwrap();
        assert_eq!("Active", result);

        //Test case 2 - power
        let request_result = client.power("kitchen", "Rosette", "Left");
        assert!(request_result.is_ok());
        let result = request_result.unwrap();
        assert_eq!("4", result);

        //Test case 2 - switch 3 times from Available to Active -> Available -> Active
        let request_result = client.switch("kitchen", "Rosette", "Left");
        assert!(request_result.is_ok());
        let result = request_result.unwrap();
        assert_eq!("Switched to Active", result);

        let request_result = client.switch("kitchen", "Rosette", "Left");
        assert!(request_result.is_ok());
        let result = request_result.unwrap();
        assert_eq!("Switched to Available", result);

        let request_result = client.switch("kitchen", "Rosette", "Left");
        assert!(request_result.is_ok());
        let result = request_result.unwrap();
        assert_eq!("Switched to Active", result);

        //Test case 3 - UDP thermometer listen and get data
        let request_result = client.get_temperature("kitchen", "Thermometer", "Main");
        assert!(request_result.is_ok());
        let first_result = request_result.unwrap();
        assert_eq!("0", &first_result);
        println!("First temperature {}", &first_result);

        let sender_udp_address = "127.0.0.1:55223";
        let sender_udp = init_sender_udp(sender_udp_address);
        send(&sender_udp, "127.0.0.1:55331", &Vec::from((16 as u16).to_be_bytes()));
        thread::sleep(Duration::from_millis(1000));
        let request_result = client.get_temperature("kitchen", "Thermometer", "Main");

        assert!(request_result.is_ok());
        assert_eq!("16", request_result.unwrap());
    }

    fn init_sender_udp(host: &str) -> net::UdpSocket {

        println!("initializing host");
        let socket = net::UdpSocket::bind(host).expect("failed to bind host socket");

        socket
    }

    fn send(socket: &net::UdpSocket, receiver: &str, msg: &Vec<u8>) -> usize {

        println!("sending message: {:?}", msg);
        let result: usize = 0;
        match socket.send_to(&msg, receiver) {
            Ok(number_of_bytes) => println!("{:?}", number_of_bytes),
            Err(fail) => println!("failed sending {:?}", fail),
        }

        result
    }
}
