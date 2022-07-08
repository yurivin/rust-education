use crate::handler::{Request, RequestHandler};
use iotp::server::{IotpConnection, IotpServer};
use smart_house::smart_house::SmartHouse;
use std::error::Error;
use std::{fs, thread};

pub fn run(addr: String) -> Result<(), Box<dyn Error>> {
    let server = IotpServer::bind(addr)?;

    for connection in server.incoming() {
        let connection = match connection {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Can't establish connection: {}", e);
                continue;
            }
        };

        let addr = match connection.peer_addr() {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".into(),
        };

        println!("New client connected: {}", addr);

        thread::spawn(move || {
            if handle_connection(connection, SmartHouse::default()).is_err() {
                println!("Client disconnected: {}", addr);
            }
        });
    }
    Ok(())
}

fn handle_connection(
    mut connection: IotpConnection,
    house: SmartHouse,
) -> Result<(), anyhow::Error> {
    let mut handler = RequestHandler::new(house);
    loop {
        let req_str = connection.recv_request()?;
        let req = Request::new(&req_str);
        connection.send_response(handler.handle(req))?;
    }
}
