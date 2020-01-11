use std::env;
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

const GPIO_LED: u8 = 17;
const ONE_LENGTH: u64 = 300;
const FIRST_PAUSE_LENGTH: u64 = 2500;
const LOW_PAUSE_LENGTH: u64 = 170;
const PULSE_PAUSE_LENGTH: u64 = 1200;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() != 3 {
        return Err("Usage [module/device] [on/off]".into());
    }

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    let m1_on: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 1, 2, 3, 2,
        2, 2, 1,
    ];
    let m1_off: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 1, 3, 2, 2,
        2, 2, 1,
    ];

    let one_on: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2,
        3, 2, 1,
    ];
    let one_off: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 2, 1, 2,
        3, 2, 1,
    ];

    let two_on: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2,
        2, 3, 1,
    ];
    let two_off: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 2, 1, 2,
        2, 3, 1,
    ];

    let three_on: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2,
        3, 1, 2,
    ];
    let three_off: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 2, 3, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 2, 1, 2,
        3, 1, 2,
    ];

    let m2_on: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 1, 2, 3, 2,
        2, 2, 1,
    ];
    let m2_off: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 1, 3, 2, 2,
        2, 2, 1,
    ];

    let four_on: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2,
        3, 2, 1,
    ];
    let four_off: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 2, 1, 2,
        3, 2, 1,
    ];

    let five_on: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2,
        2, 3, 1,
    ];
    let five_off: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 2, 1, 2,
        2, 3, 1,
    ];

    let six_on: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 1, 2, 2,
        3, 1, 2,
    ];
    let six_off: [i32; 33] = [
        2, 1, 2, 2, 3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 2, 2, 3, 1, 3, 2, 1, 3, 1, 3, 2, 2, 1, 2,
        3, 1, 2,
    ];

    let first = &args[1];

    match first.as_ref() {
        "all" if (&args[2] == "on") => {
            send_code(&m1_on, &mut pin);
            send_code(&m2_on, &mut pin)
        }
        "all" if (&args[2] == "off") => {
            send_code(&m1_off, &mut pin);
            send_code(&m2_off, &mut pin)
        }
        "m1" if (&args[2] == "on") => send_code(&m1_on, &mut pin),
        "m1" if (&args[2] == "off") => send_code(&m1_off, &mut pin),
        "m2" if (&args[2] == "on") => send_code(&m2_on, &mut pin),
        "m2" if (&args[2] == "off") => send_code(&m2_off, &mut pin),
        "1" if (&args[2] == "on") => send_code(&one_on, &mut pin),
        "1" if (&args[2] == "off") => send_code(&one_off, &mut pin),
        "2" if (&args[2] == "on") => send_code(&two_on, &mut pin),
        "2" if (&args[2] == "off") => send_code(&two_off, &mut pin),
        "3" if (&args[2] == "on") => send_code(&three_on, &mut pin),
        "3" if (&args[2] == "off") => send_code(&three_off, &mut pin),
        "4" if (&args[2] == "on") => send_code(&four_on, &mut pin),
        "4" if (&args[2] == "off") => send_code(&four_off, &mut pin),
        "5" if (&args[2] == "on") => send_code(&five_on, &mut pin),
        "5" if (&args[2] == "off") => send_code(&five_off, &mut pin),
        "6" if (&args[2] == "on") => send_code(&six_on, &mut pin),
        "6" if (&args[2] == "off") => send_code(&six_off, &mut pin),
        _ => println!("Unknown"),
    }

    Ok(())
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
