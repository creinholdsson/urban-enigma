use rollo_rs::rollo;
use rppal::gpio::Gpio;
use std::env;
use std::sync::{Arc, Mutex};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{0:?}", args);
    let code = args[1].clone();
    const GPIO_LED: u8 = 17;
    let pin = Arc::new(Mutex::new(
        Gpio::new().unwrap().get(GPIO_LED).unwrap().into_output(),
    ));

    println!("Writing {}", code);

    let device = rollo::Rollo::new(&code, pin);

    let direction = match args[2].as_str() {
        "u" => rollo::Direction::UP,
        "d" => rollo::Direction::DOWN,
        _ => rollo::Direction::PAUSE,
    };

    for _ in 0..3 {
        device.send(direction.clone());
    }
    println!("Sent");
}
