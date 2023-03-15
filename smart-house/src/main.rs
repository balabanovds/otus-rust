#![allow(dead_code)]

pub struct Socket {
    power_consumption: u16,
    is_switched_on: bool,
}

impl Socket {
    pub fn new(power: u16) -> Self {
        Socket {
            power_consumption: power,
            is_switched_on: true,
        }
    }

    pub fn description(&self) -> String {
        let status = match self.is_switched_on {
            true => "switched on",
            false => "switched off",
        };
        format!(
            "This is Socket: power consumtion is {} W, status is {}",
            self.power_consumption, status
        )
    }

    pub fn on(&mut self) {
        self.is_switched_on = true;
    }

    pub fn off(&mut self) {
        self.is_switched_on = false;
    }

    pub fn current_power(&self) -> u16 {
        self.power_consumption
    }
}

pub struct Thermometer {
    current_temp: f64,
}

impl Thermometer {
    pub fn new(temp: f64) -> Self {
        Thermometer { current_temp: temp }
    }

    pub fn temperature(&self) -> f64 {
        self.current_temp
    }

    fn set_temp(&mut self, new_temp: f64) {
        self.current_temp = new_temp;
    }
}

fn main() {
    let mut socket = Socket::new(42);

    println!("{}", socket.description());

    socket.off();

    println!("{}", socket.description());

    let mut term = Thermometer::new(36.6);
    term.set_temp(37.0);
    println!("Current temp is {}", term.temperature());
}
