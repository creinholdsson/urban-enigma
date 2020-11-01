use rppal::gpio::OutputPin;
use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

const PULSE_LENGTH: u64 = 250; //unit us
const NEXA_CHANNEL: &str = "11";

// Nexa protocol taken from
// Bit pattern: S HHHH HHHH HHHH HHHH HHHH HHHH HHGO CCEE P
// s: sync
// h: sender id
// g: group 0 on 1 off
// o: on/off, 0 on 1 off
// c: channel, nexa= 00
// e: unit, nexa 1=11, 2 = 01, 3 = 10
// p: pause
// GOCCEE
// 123456

#[derive(Clone)]
pub struct Nexa<'a> {
    pub sender_id: &'a str,
    pin: Arc<Mutex<OutputPin>>,
}

#[derive(Clone, Copy)]
pub enum DeviceNumber {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy)]
pub enum DeviceMode {
    On,
    Off,
}

impl fmt::Display for DeviceMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DeviceMode::On => write!(f, "0"),
            DeviceMode::Off => write!(f, "1"),
        }
    }
}

impl Nexa<'_> {
    pub fn new(sender_id: &str, pin: Arc<Mutex<OutputPin>>) -> Nexa {
        assert!(sender_id.len() == 26);
        Nexa { sender_id, pin }
    }

    fn get_code(&self, whole_group: bool, device_no: DeviceNumber, mode: DeviceMode) -> String {
        let device_id = match device_no {
            DeviceNumber::One => "11",
            DeviceNumber::Two => "01",
            DeviceNumber::Three => "10",
        };
        let group_control = match whole_group {
            true => '0',
            false => '1',
        };
        format!(
            "{}{}{}{}{}",
            self.sender_id, group_control, mode, NEXA_CHANNEL, device_id
        )
    }

    pub fn turn_device_on(&self, device_no: DeviceNumber) {
        println!("turning device on");
        for _ in 0..5 {
            let code = self.get_code(false, device_no, DeviceMode::On);
            self.write_code(&code)
        }
    }

    pub fn turn_device_off(&self, device_no: DeviceNumber) {
        let code = self.get_code(false, device_no, DeviceMode::Off);
        for _ in 0..5 {
            self.write_code(&code)
        }
    }

    pub fn turn_group_off(&self) {
        let code = self.get_code(true, DeviceNumber::One, DeviceMode::Off);
        for _ in 0..5 {
            self.write_code(&code);
        }
    }

    pub fn turn_group_on(&self) {
        let code = self.get_code(true, DeviceNumber::One, DeviceMode::On);
        for _ in 0..5 {
            self.write_code(&code);
        }
    }

    fn write_code(&self, code: &str) {
        assert!(code.len() == 32);
        self.send_sync();
        for c in code.chars() {
            match c {
                '1' => self.send_one(),
                '0' => self.send_zero(),
                _ => panic!("Illegal code"),
            }
        }
        self.send_pause();
    }

    fn send_zero(&self) {
        self.send_physical_zero();
        self.send_physical_one();
    }

    fn send_one(&self) {
        self.send_physical_one();
        self.send_physical_zero();
    }

    fn send_physical_one(&self) {
        let mut pin = self.pin.lock().unwrap();
        pin.set_high();
        thread::sleep(Duration::from_micros(PULSE_LENGTH));
        pin.set_low();
        thread::sleep(Duration::from_micros(PULSE_LENGTH));
    }

    fn send_physical_zero(&self) {
        let mut pin = self.pin.lock().unwrap();
        pin.set_high();
        thread::sleep(Duration::from_micros(PULSE_LENGTH));
        pin.set_low();
        thread::sleep(Duration::from_micros(5 * PULSE_LENGTH));
    }

    fn send_sync(&self) {
        let mut pin = self.pin.lock().unwrap();
        pin.set_high();
        thread::sleep(Duration::from_micros(PULSE_LENGTH));
        pin.set_low();
        thread::sleep(Duration::from_micros(10 * PULSE_LENGTH));
    }

    fn send_pause(&self) {
        let mut pin = self.pin.lock().unwrap();
        pin.set_high();
        thread::sleep(Duration::from_micros(PULSE_LENGTH));
        pin.set_low();
        thread::sleep(Duration::from_micros(40 * PULSE_LENGTH));
    }
}
