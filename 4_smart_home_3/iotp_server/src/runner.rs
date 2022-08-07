use crate::handler::{Request, RequestHandler};
use iotp::server::{IotpConnection, IotpServer};
use smart_house::smart_house::SmartHouse;
use std::error::Error;

#[tokio::main]
pub async fn run(addr: String) -> Result<(), Box<dyn Error>> {
    let server = IotpServer::bind(addr).await?;

    loop {
        let connection = match server.accept().await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Can't establish connection: {}", e);
                continue;
            }
        };

        let addr = match connection.peer_addr().await {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".into(),
        };

        println!("New client connected: {}", addr);

        tokio::spawn(async move {
            let mut smart_house = SmartHouse::default();
            let thermometre = smart_house.store.get_mut("kitchenThermometerMain");

            thermometre
                .unwrap()
                .device
                .listen("127.0.0.1:55331".to_owned());

            if handle_connection(connection, smart_house).await.is_err() {
                println!("Client disconnected: {}", addr);
            }
        });
    }
}

async fn handle_connection(
    mut connection: IotpConnection,
    house: SmartHouse,
) -> Result<(), anyhow::Error> {
    let mut handler = RequestHandler::new(house);
    loop {
        let req_str = connection.recv_request().await?;
        let req = Request::new(&req_str);
        connection.send_response(handler.handle(req)).await?;
    }
}
