trait SocketTrait {
    fn info() -> &'static str;
    fn on(&self) -> bool;
    fn off(&self) -> bool;
    fn power_status(&self) -> u64;
}

struct SmartSocket {
    on: bool,
}

impl SocketTrait for SmartSocket {
    fn info() -> &'static str {
        "This is a Rosette. you can Turn it on/off using on() & off() functions. \
        Also you get get data about current power usage using status() function"
    }

    /// Returns the resulting status of the Rossette if it ON or OFF
    fn on(&self) -> bool {
        todo!()
    }
    /// Returns the resulting status of the Rossette if it ON or OFF
    fn off(&self) -> bool {
        todo!()
    }
    /// Returns currently used power
    fn power_status(&self) -> u64 {
        todo!()
    }
}

trait ThermometrTrait {
    fn temperature(&self) -> f32;
}

struct Thermometre {}

impl ThermometrTrait for Thermometre {
    fn temperature(&self) -> f32 {
        todo!()
    }
}

fn main() {
    println!("Homework 2");

    let socket = SmartSocket { on: false };
    socket.on();
    socket.off();
    SmartSocket::info();

    println!("Socket is {0}", socket.on);
}
