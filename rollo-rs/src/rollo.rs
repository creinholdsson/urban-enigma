use rppal::gpio::OutputPin;
use std::sync::{Arc, Mutex};

const PULSE_LENGTH: u64 = 250;
#[derive(Clone)]
pub struct Rollo<'a> {
    code: &'a str,
    pin: Arc<Mutex<OutputPin>>,
}
#[derive(Clone)]
pub enum Direction {
    UP,
    PAUSE,
    DOWN,
}

impl Rollo<'_> {
    pub fn new(code: &str, pin: Arc<Mutex<OutputPin>>) -> Rollo {
        Rollo { code, pin }
    }

    fn send_t0(&self) {
        self.transmit(1, 3);
        self.transmit(1, 3);
    }

    fn send_tf(&self) {
        self.transmit(1, 3);
        self.transmit(3, 1);
    }

    pub fn send(&self, direction: Direction) {
        for _ in 0..6 {
            self.send_sync();
            let full = self.code.to_string()
                + match direction {
                    Direction::UP => "F0F",
                    Direction::PAUSE => "FFF",
                    Direction::DOWN => "101",
                };
            for c in full.chars() {
                match c {
                    '0' => self.send_t0(),
                    'F' => self.send_tf(),
                    '1' => self.send_t1(),
                    'Q' => self.send_qq(),
                    _ => panic!("Should not happen!"),
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_micros(PULSE_LENGTH));
    }

    fn send_sync(&self) {
        self.transmit(18, 6);
    }

    fn transmit(&self, high_pulses: u64, low_pulses: u64) {
        let mut pin = self.pin.lock().unwrap();
        pin.set_high();
        std::thread::sleep(std::time::Duration::from_micros(PULSE_LENGTH * high_pulses));
        pin.set_low();
        std::thread::sleep(std::time::Duration::from_micros(PULSE_LENGTH * low_pulses));
    }

    fn send_t1(&self) {
        self.transmit(3, 1);
        self.transmit(3, 1);
    }

    fn send_qq(&self) {
        self.transmit(3, 1);
        self.transmit(1, 3);
    }
}
