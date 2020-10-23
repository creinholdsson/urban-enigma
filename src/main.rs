#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use log::{info, warn};
extern crate log4rs;

use std::thread;

use rocket::config::{Config, Environment};
use rocket_contrib::serve::StaticFiles;

use std::error::Error;

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

use std::time::Duration;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;

const GPIO_LED: u8 = 17;
const ONE_LENGTH: u64 = 300;
const FIRST_PAUSE_LENGTH: u64 = 2500;
const LOW_PAUSE_LENGTH: u64 = 170;
const PULSE_PAUSE_LENGTH: u64 = 1200;

const M1_ON: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 1, 2, 3, 2, 2, 2,
    1,
];
const M1_OFF: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 1, 3, 2, 2, 2, 2,
    1,
];

const ONE_ON: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2, 3, 2,
    1,
];
const ONE_OFF: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 2, 1, 2, 3, 2,
    1,
];

const TWO_ON: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 3,
    1,
];
const TWO_OFF: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 2, 1, 2, 2, 3,
    1,
];

const THREE_ON: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2, 3, 1, 2,
];
const THREE_OFF: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 2, 1, 2, 3, 1,
    2,
];

const M2_ON: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 1, 2, 3, 2, 2, 2, 1,
];
const M2_OFF: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 1, 3, 2, 2, 2, 2, 1,
];

const FOUR_ON: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2, 3, 2, 1,
];
const FOUR_OFF: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 2, 1, 2, 3, 2, 1,
];

const FIVE_ON: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 3, 1,
];
const FIVE_OFF: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 2, 1, 2, 2, 3, 1,
];

const SIX_ON: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2, 3, 1, 2,
    
];
const SIX_OFF: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 2, 1, 2, 3, 1, 2,
];

const SEVEN_ON: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2, 3, 2, 1,

];
const SEVEN_OFF: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 1, 2, 3, 2, 1,
];

const EIGHT_ON: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 3, 1,
];
const EIGHT_OFF: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 1, 2, 2, 3, 1,
];

const NINE_ON: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2, 3, 1, 2,
];
const NINE_OFF: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 1, 2, 3, 1, 2,
];

const M3_ON: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 1, 2, 3, 2, 2, 2, 1,
];

const M3_OFF: [i32; 33] = [
    2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 1, 3, 2, 2, 2, 2, 1
];


fn set_device_mode(device_name: &str, mode: &str) -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    match device_name {
        "all" if (mode == "on") => {
            send_code(&M1_ON, &mut pin);
            send_code(&M2_ON, &mut pin);
	    send_code(&M3_ON, &mut pin)
        }
        "all" if (mode == "off") => {
            send_code(&M1_OFF, &mut pin);
            send_code(&M2_OFF, &mut pin);
            send_code(&M3_OFF, &mut pin)
        }
        "m1" if (mode == "on") => send_code(&M1_ON, &mut pin),
        "m1" if (mode == "off") => send_code(&M1_OFF, &mut pin),
        "m2" if (mode == "on") => send_code(&M2_ON, &mut pin),
        "m2" if (mode == "off") => send_code(&M2_OFF, &mut pin),
	"m3" if (mode == "on") => send_code(&M3_ON, &mut pin),
        "m3" if (mode == "off") => send_code(&M3_OFF, &mut pin),
        "1" if (mode == "on") => send_code(&ONE_ON, &mut pin),
        "1" if (mode == "off") => send_code(&ONE_OFF, &mut pin),
        "2" if (mode == "on") => send_code(&TWO_ON, &mut pin),
        "2" if (mode == "off") => send_code(&TWO_OFF, &mut pin),
        "3" if (mode == "on") => send_code(&THREE_ON, &mut pin),
        "3" if (mode == "off") => send_code(&THREE_OFF, &mut pin),
        "4" if (mode == "on") => send_code(&FOUR_ON, &mut pin),
        "4" if (mode == "off") => send_code(&FOUR_OFF, &mut pin),
        "5" if (mode == "on") => send_code(&FIVE_ON, &mut pin),
        "5" if (mode == "off") => send_code(&FIVE_OFF, &mut pin),
        "6" if (mode == "on") => send_code(&SIX_ON, &mut pin),
        "6" if (mode == "off") => send_code(&SIX_OFF, &mut pin),
	"7" if (mode == "on") => send_code(&SEVEN_ON, &mut pin),
	"7" if (mode == "off") => send_code(&SEVEN_OFF, &mut pin),
	"8" if (mode == "on") => send_code(&EIGHT_ON, &mut pin),
	"8" if (mode == "off") => send_code(&EIGHT_OFF, &mut pin),
	"9" if (mode == "on") => send_code(&NINE_ON, &mut pin),
	"9" if (mode == "off") => send_code(&NINE_OFF, &mut pin),
        _ => println!("Unknown"),
    }
    Ok(())
}

#[get("/<device>?<mode>&<delay>")]
fn set_device(device: String, mode: String, delay: Option<u64>) -> String {
    match delay {
        Some(x) if x > 0 => {
            info!("Delay was set to {} for {}", x, device);
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(x));
                match set_device_mode(device.as_ref(), "off") {
                    Ok(_) => info!("Device {} was turned off", device),
                    Err(x) => warn!("Could not turn {} off ({})", device, x),
                }
            });
        "Success".to_string()
        },
        None | Some(_) => {
            info!("Setting {} to {}", device, mode);
            match set_device_mode(device.as_ref(), mode.as_ref()) {
                Ok(_) => "Success".to_string(),
                Err(x) => x.to_string(),
            }
        }
    }
}

fn main() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("urban-enigma.log")
        .unwrap();

    let log_config = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            log4rs::config::Root::builder()
                .appender("logfile")
                .build(LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(log_config).unwrap();

    let config = Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(80)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/api/set", routes![set_device])
        .mount("/", StaticFiles::from("/home/pi/home-automation/"))
        .launch();
}

fn send_code(code: &[i32], pin: &mut OutputPin) {
    for _x in 0..5 {
        send_one(pin);
        thread::sleep(Duration::from_micros(FIRST_PAUSE_LENGTH));
        for _i in code {
            if *_i == 1 {
                send_one(pin);
            } else if *_i == 2 {
                send_two(pin);
            }
            if *_i == 3 {
                send_three(pin);
            }
            thread::sleep(Duration::from_micros(PULSE_PAUSE_LENGTH));
        }
        thread::sleep(Duration::from_millis(200));
    }
}

fn send_one(pin: &mut OutputPin) {
    pin.set_high();
    thread::sleep(Duration::from_micros(ONE_LENGTH));
    pin.set_low();
}

fn send_two(pin: &mut OutputPin) {
    send_one(pin);
    thread::sleep(Duration::from_micros(LOW_PAUSE_LENGTH));
    send_one(pin);
}

fn send_three(pin: &mut OutputPin) {
    send_one(pin);
    thread::sleep(Duration::from_micros(LOW_PAUSE_LENGTH));
    send_one(pin);
    thread::sleep(Duration::from_micros(LOW_PAUSE_LENGTH));
    send_one(pin);
}
