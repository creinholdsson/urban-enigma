use rusqlite::{params, Connection, Error, Result, NO_PARAMS};
use serde::Serialize;
use std::fs;

#[derive(Clone, Debug, Serialize)]
pub struct Device {
    id: i64,
    name: String,
    group_id: i32,
    current_state: bool,
}

#[derive(Default, Clone)]
pub struct Repo {
    connection_string: String,
}

impl Device {
    pub fn new(name: &str, group_id: i32, current_state: bool) -> Device {
        Device {
            id: 0,
            name: String::from(name),
            group_id,
            current_state,
        }
    }
}

impl Repo {
    pub fn new(connection_string: &str) -> Repo {
        Repo {
            connection_string: String::from(connection_string),
        }
    }

    pub fn assure_created(&self) -> Result<bool> {
        let conn = Connection::open(&self.connection_string)?;

        let mut statement =
            conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='devices'")?;
        match statement.exists(NO_PARAMS) {
            Ok(x) => {
                if x {
                    return Ok(true);
                }
            }
            Err(err) => return Err(err),
        }

        println!("Table not existing, creating");

        match conn.execute(
            "
    		CREATE TABLE devices (
				id INTEGER PRIMARY KEY AUTOINCREMENT,
				name VARCHAR(100) NOT NULL,
				group_id INTEGER NOT NULL,
				current_state BIT NOT NULL DEFAULT 0
			)
    		",
            params![],
        ) {
            Ok(_) => return Ok(true),
            Err(err) => {
                return Err(err);
            }
        }
    }

    pub fn get_devices(&self) -> Result<Vec<Device>> {
        let conn = Connection::open(&self.connection_string)?;

        let mut statement =
            conn.prepare("SELECT id, name, group_id, current_state FROM devices")?;

        let mut result: Vec<Device> = vec![];

        let device_iter = statement.query_map(NO_PARAMS, |row| {
            Ok(Device {
                id: row.get(0)?,
                name: row.get(1)?,
                group_id: row.get(2)?,
                current_state: row.get(3)?,
            })
        })?;

        device_iter.for_each(|d| result.push(d.unwrap()));

        Ok(result)
    }

    pub fn get_device(&self, id: i64) -> Result<Option<Device>> {
        let conn = Connection::open(&self.connection_string)?;

        return match conn.query_row(
            "SELECT id, name, group_id, current_state FROM devices WHERE id = ?1",
            params![id],
            |row| {
                Ok(Device {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    group_id: row.get(2)?,
                    current_state: row.get(3)?,
                })
            },
        ) {
            Ok(x) => Ok(Some(x)),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(err) => {
                log::error!("Error: {}", err);
                println!("err: {}", err);
                return Err(err);
            }
        };
    }

    pub fn get_group(&self, group_id: i32) -> Result<Vec<Device>> {
        let conn = Connection::open(&self.connection_string)?;

        let mut statement =
            conn.prepare("SELECT id, name, group_id, current_state FROM devices WHERE id = ?1")?;

        let device_iter = statement.query_map(params![group_id], |row| {
            Ok(Device {
                id: row.get(0)?,
                name: row.get(1)?,
                group_id: row.get(2)?,
                current_state: row.get(3)?,
            })
        })?;

        let mut result: Vec<Device> = vec![];

        for device in device_iter {
            result.push(device.unwrap());
        }

        Ok(result)
    }

    pub fn add_device(&self, device: &mut Device) -> Result<bool> {
        let conn = Connection::open(&self.connection_string)?;
        let mut statement = conn.prepare("INSERT INTO devices(name, group_id) VALUES(?1, ?2);")?;

        match statement.insert(params![device.name, device.group_id]) {
            Ok(id) => {
                device.id = id;
                return Ok(true);
            }
            Err(err) => {
                println!("insert err: {}", err);
                return Err(err);
            }
        }
    }

    pub fn update_device(&self, device: &Device) -> Result<bool> {
        let conn = Connection::open(&self.connection_string)?;
        let mut statement =
            conn.prepare("UPDATE devices SET name = ?1, current_state = ?2 WHERE id = ?3")?;

        return match statement.execute(params![device.name, device.current_state, device.id]) {
            Ok(_) => Ok(true),
            Err(err) => Err(err),
        };
    }

    pub fn update_devices(&self, devices: &[Device]) -> Result<bool> {
        let conn = Connection::open(&self.connection_string)?;
        let mut statement =
            conn.prepare("UPDATE devices SET name = ?1, current_state = ?2 WHERE id = ?3")?;

        for device in devices.iter() {
            match statement.execute(params![device.name, device.current_state, device.id]) {
                Ok(_) => {}
                Err(err) => return Err(err),
            }
        }
        Ok(true)
    }
}

#[test]
fn test_device_empty_database() {
    let repo = Repo::new("test.db");
    repo.assure_created().unwrap();
    let device = repo.get_device(2);

    assert!(device.is_ok());
    assert!(device.unwrap().is_none());

    fs::remove_file("test.db").unwrap();
}

#[test]
fn test_non_existing_database() {
    let repo = Repo::new("test.db");
    let created = repo.assure_created();
    let created2 = repo.assure_created();

    assert!(created.is_ok());
    assert!(created2.is_ok());
    assert!(created.unwrap() == true);
    assert!(created2.unwrap() == true);

    fs::remove_file("test.db").unwrap();
}

#[test]
fn test_insert_device() {
    let mut device = Device::new("test", 1, false);

    let repo = Repo::new("test.db");
    repo.assure_created().unwrap();

    let inserted = repo.add_device(&mut device);

    assert!(inserted.is_ok());

    assert!(device.id != 0);
    assert!(device.name == "test");
    assert!(device.group_id == 1);

    let get_result = repo.get_device(device.id);
    assert!(get_result.is_ok());

    let inserted_device = get_result.unwrap();
    assert!(inserted_device.is_some());

    let get_group = repo.get_group(device.group_id);
    assert!(get_group.is_ok());

    let group = get_group.unwrap();

    assert!(1 == group.len());

    fs::remove_file("test.db").unwrap();
}

#[test]
fn test_update_device() {
    let mut device = Device::new("test", 1, false);

    let repo = Repo::new("test.db");
    repo.assure_created().unwrap();

    let inserted = repo.add_device(&mut device);

    assert!(inserted.is_ok());

    device.current_state = true;

    let update_result = repo.update_device(&device);

    assert!(update_result.is_ok());

    let updated = repo.get_device(device.id);

    let updated_device = updated.unwrap().unwrap();

    assert!(true == updated_device.current_state);
    fs::remove_file("test.db").unwrap();
}

#[test]
fn test_get_devices() {
    let mut device1 = Device::new("test1", 1, false);
    let mut device2 = Device::new("test2", 1, false);

    let repo = Repo::new("test.db");

    repo.assure_created().unwrap();

    assert!(repo.add_device(&mut device1).is_ok());
    assert!(repo.add_device(&mut device2).is_ok());

    let devices = repo.get_devices().unwrap();

    assert!(2 == devices.len());
}

#[test]
fn test_get_devices_in_group() {
    let mut device1 = Device::new("test1", 1, false);
    let mut device2 = Device::new("test2", 2, false);

    let repo = Repo::new("test.db");

    repo.assure_created().unwrap();

    assert!(repo.add_device(&mut device1).is_ok());
    assert!(repo.add_device(&mut device2).is_ok());

    let group_devices = repo.get_group(1).unwrap();

    assert!(1 == group_devices.len())
}
