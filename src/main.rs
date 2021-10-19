#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use log::{error, info, warn, trace};
extern crate log4rs;

use std::thread;
mod nexa;
mod repo;

use rocket::config::{Config, Environment};
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use std::error::Error;
use rppal::gpio::Gpio;
use std::time::Duration;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use std::sync::{Arc, Mutex};

use rocket::State;

#[derive(Clone)]
struct SenderState<'a> {
    sender_one: nexa::Nexa<'a>,
    sender_two: nexa::Nexa<'a>,
    sender_three: nexa::Nexa<'a>,
    sender_four: nexa::Nexa<'a>,
    repo: repo::Repo<'a>,
}

fn call_external_device(base_url: &str, device_id: &str, mode: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/{}/{}", base_url, device_id, mode);

    match ureq::get(&url).call() {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::new(err)),
    }
}

fn get_device_number_from_id<'b, 'a>(
    id: &'b str,
    state: &'a SenderState,
) -> Option<(&'a nexa::Nexa<'a>, nexa::DeviceNumber)> {
    match id {
        "1" => Some((&state.sender_one, nexa::DeviceNumber::One)),
        "2" => Some((&state.sender_one, nexa::DeviceNumber::Two)),
        "3" => Some((&state.sender_one, nexa::DeviceNumber::Three)),
        "4" => Some((&state.sender_two, nexa::DeviceNumber::One)),
        "5" => Some((&state.sender_two, nexa::DeviceNumber::Two)),
        "6" => Some((&state.sender_two, nexa::DeviceNumber::Three)),
        "7" => Some((&state.sender_three, nexa::DeviceNumber::One)),
        "8" => Some((&state.sender_three, nexa::DeviceNumber::Two)),
        "9" => Some((&state.sender_three, nexa::DeviceNumber::Three)),
        "10" => Some((&state.sender_four, nexa::DeviceNumber::One)),
        _ => None,
    }
}

fn set_device_mode(
    device_name: &str,
    mode: &str,
    sender_state: &SenderState,
) -> Result<(), Box<dyn Error>> {
    if let Some((device, device_number)) = get_device_number_from_id(device_name, sender_state) {
        match mode {
            "on" => device.turn_device_on(device_number),
            "off" => device.turn_device_off(device_number),
            _ => {}
        }
    }
    match device_name {
        "all" if (mode == "on") => {
            sender_state.sender_one.turn_group_on();
            sender_state.sender_two.turn_group_on();
            sender_state.sender_three.turn_group_on();
        }
        "all" if (mode == "off") => {
            sender_state.sender_one.turn_group_off();
            sender_state.sender_two.turn_group_off();
            sender_state.sender_three.turn_group_off();
        }
        "m1" if (mode == "on") => sender_state.sender_one.turn_group_on(),
        "m1" if (mode == "off") => sender_state.sender_one.turn_group_off(),
        "m2" if (mode == "on") => sender_state.sender_two.turn_group_on(),
        "m2" if (mode == "off") => sender_state.sender_two.turn_group_off(),
        "m3" if (mode == "on") => sender_state.sender_two.turn_group_off(),
        "m3" if (mode == "off") => sender_state.sender_two.turn_group_off(),
        "11" => call_external_device("http://192.168.10.124", "4", mode).unwrap(),
        "12" => call_external_device("http://192.168.10.124", "5", mode).unwrap(),

        _ => println!("Unknown"),
    }
    Ok(())
}

#[get("/<device>?<mode>&<delay>")]
fn set_device(
    device: String,
    mode: String,
    delay: Option<u64>,
    sender_state: State<SenderState>,
) -> String {
    match delay {
        Some(x) if x > 0 => {
            info!("Delay was set to {} for {}", x, device);
            let sender = sender_state.inner().clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(x));
                match set_device_mode(device.as_ref(), "off", &sender) {
                    Ok(_) => info!("Device {} was turned off", device),
                    Err(x) => warn!("Could not turn {} off ({})", device, x),
                }
            });
            "Success".to_string()
        }
        None | Some(_) => {
            info!("Setting {} to {}", device, mode);
            match set_device_mode(
                device.as_ref(),
                mode.as_ref(),
                &sender_state.inner().clone(),
            ) {
                Ok(_) => "Success".to_string(),
                Err(x) => x.to_string(),
            }
        }
    }
}

#[post("/<device_id>?<mode>&<delay>")]
fn post_device(
    device_id: i64,
    mode: String,
    delay: Option<u64>,
    sender_state: State<SenderState>,
) -> Option<Json<repo::Device>> {
    return match sender_state.repo.get_device(device_id).as_mut() {
        Ok(None) => return None,
        Ok(Some(device)) => {
            return match delay {
                Some(x) if x > 0 => {
                    let sender = sender_state.inner().clone();
                    let device_str: String = device_id.to_string();
                    let d = device.clone();
                    thread::spawn(move || {
                        thread::sleep(Duration::from_secs(x));
                        match set_device_mode(&device_str, "off", &sender) {
                            Ok(_) => {
                                sender.repo.update_device(&d).unwrap_or_else(|e| {
                                    error!("Failed to update {}", e);
                                    true
                                });
                            }
                            Err(err) => {
                                error!(
                                    "Could not turn {} off with delay ({}) {}",
                                    device_str, x, err
                                );
                                // true
                            }
                        }
                    });
                    Some(Json(device.clone()))
                }
                None | Some(_) => match mode.as_ref() {
                    "on" => {
                        device.current_state = true;
                        sender_state.repo.update_device(device).unwrap();
                        set_device_mode(device_id.to_string().as_ref(), "on", &sender_state)
                            .unwrap();
                        Some(Json(device.clone()))
                    }
                    "off" => {
                        device.current_state = false;
                        sender_state.repo.update_device(device).unwrap();
                        set_device_mode(device_id.to_string().as_ref(), "off", &sender_state)
                            .unwrap();
                        Some(Json(device.clone()))
                    }
                    _ => None,
                },
            };
        }
        Err(x) => {
            error!("Error: {}", x);
            None
        }
    };
}

#[get("/")]
fn get_devices(sender_state: State<SenderState>) -> Json<Vec<repo::Device>> {
    Json(sender_state.repo.get_devices().unwrap())
}

fn periodic_state_publish(sender: SenderState) {
    loop {
        if let Ok(devices) = sender.repo.get_devices() {
            for device in devices.iter() {
                let device_name = device.id.to_string();
                if let Some((sender, device_number)) =
                get_device_number_from_id(&device_name, &sender)
                {
                    match device.current_state {
                        true => sender.turn_device_on(device_number),
                        false => sender.turn_device_off(device_number),
                    }
                }
            }
        }
        trace!("Sending periodic update");
        thread::sleep(Duration::from_secs(120));
    }
}

fn main() {
    const GPIO_LED: u8 = 17;
    let pin = Arc::new(Mutex::new(
        Gpio::new().unwrap().get(GPIO_LED).unwrap().into_output(),
    ));

    let repo = repo::Repo::new("/home/pi/test.db");
    repo.assure_created().unwrap();

    let nexa_state = SenderState {
        sender_one: nexa::Nexa::new("11000000000000000000000010", Arc::clone(&pin)), //50331650
        sender_two: nexa::Nexa::new("11000000000000000000000001", Arc::clone(&pin)), //50331649
        sender_three: nexa::Nexa::new("11000000000000000000000000", Arc::clone(&pin)), // 50331648
        sender_four: nexa::Nexa::new("11000000000000000000000011", Arc::clone(&pin)), // 50331651
        repo,
    };

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

    let state2 = nexa_state.clone();
    thread::spawn(move || periodic_state_publish(state2));

    let config = Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(80)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .manage(nexa_state)
        .mount("/api/set", routes![set_device, post_device])
        .mount("/api/", routes![get_devices])
        .mount("/", StaticFiles::from("/home/pi/home-automation/"))
        .launch();
}
