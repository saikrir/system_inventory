use cache::CacheClient;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::{io::Error, net::Ipv4Addr, str::FromStr};
mod cache;

#[derive(Debug, Serialize, Deserialize)]
enum SystemType {
    Laptop { maufacturer: String },
    LowPoweredDevice { maufacturer: String },
    PC { maufacturer: String },
    Table { maufacturer: String },
}

#[derive(Debug, Serialize, Deserialize)]
struct CPUInfo {
    vendor: String,
    model: String,
    speed_in_ghz: f32,
}

impl CPUInfo {
    fn new(vendor: String, model: String, speed_in_ghz: f32) -> Self {
        CPUInfo {
            vendor,
            model,
            speed_in_ghz,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OS {
    name: String,
    family: String,
    version: f32,
}

impl OS {
    fn new(name: String, family: String, version: f32) -> Self {
        OS {
            name,
            family,
            version,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct System {
    name: String,
    system_type: SystemType,
    cpu_info: CPUInfo,
    ram_in_gb: f32,
    os: OS,
    ip_address: Ipv4Addr,
}

impl System {
    fn new(
        name: String,
        system_type: SystemType,
        cpu_info: CPUInfo,
        ram_in_gb: f32,
        os: OS,
        ip_address: Ipv4Addr,
    ) -> Self {
        System {
            name,
            system_type,
            cpu_info,
            ram_in_gb,
            os,
            ip_address,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct HomeLab {
    name: String,
    systems: Vec<System>,
}

impl HomeLab {
    fn new(name: String) -> Self {
        HomeLab {
            name,
            systems: vec![],
        }
    }

    fn add_system(&mut self, system: System) {
        self.systems.push(system)
    }

    fn search_system(&self, search_term: &str) -> Result<Vec<&System>, Error> {
        if search_term.len() < 3 {
            return Err(Error::other(
                "search term cannot be less than 3 characters".to_string(),
            ));
        }

        let results = self
            .systems
            .iter()
            .filter(|s| s.name.contains(search_term))
            .collect::<Vec<&System>>();

        if results.len() == 0 {
            Err(Error::other("No results found"))
        } else {
            Ok(results)
        }
    }
}

fn main() {
    let mut my_lab = HomeLab::new("Sai Katterishetty's Home Lab".to_string());

    my_lab.add_system(System::new(
        "sais-air.lan".to_string(),
        SystemType::Laptop {
            maufacturer: "Apple".to_string(),
        },
        CPUInfo::new("Apple".to_string(), "M3".to_string(), 4.5f32),
        24f32,
        OS::new("Sanoma".to_string(), "Macinosh".to_string(), 14.5f32),
        Ipv4Addr::from_str("192.168.86.41").unwrap(),
    ));

    my_lab.add_system(System::new(
        "sais-mac-studio".to_string(),
        SystemType::Laptop {
            maufacturer: "Apple".to_string(),
        },
        CPUInfo::new("Apple".to_string(), "M1 Max".to_string(), 4.5f32),
        32f32,
        OS::new("Sanoma".to_string(), "Macinosh".to_string(), 14.5f32),
        Ipv4Addr::from_str("192.168.86.26").unwrap(),
    ));

    my_lab.add_system(System::new(
        "skrao-lin-ws".to_string(),
        SystemType::Laptop {
            maufacturer: "LG".to_string(),
        },
        CPUInfo::new(
            "Intel".to_string(),
            "13th Gen Intel(R) Core(TM) i7-1360P".to_string(),
            2.0f32,
        ),
        32f32,
        OS::new("Zorin OS".to_string(), "Linux".to_string(), 17.0f32),
        Ipv4Addr::from_str("192.168.86.91").unwrap(),
    ));

    my_lab.add_system(System::new(
        "skrao-windows-11-pc".to_string(),
        SystemType::Laptop {
            maufacturer: "Lenovo".to_string(),
        },
        CPUInfo::new(
            "Intel".to_string(),
            "11th gen intel i7-1160g7".to_string(),
            1.22f32,
        ),
        16f32,
        OS::new("Windows".to_string(), "Windows".to_string(), 11.0f32),
        Ipv4Addr::from_str("192.168.86.51").unwrap(),
    ));

    let cache = CacheClient {
        redis_client: &redis::Client::open("redis://skrao-db-server")
            .expect("failed to connect to redis server"),
    };

    let lab_key = "myLab".to_string();

    let value = match to_string_pretty(&my_lab) {
        Ok(str_json) => {
            let cache_write_result = cache.add_to_cache(&lab_key, &str_json);
            match cache_write_result {
                Ok(_) => cache.get_cached(&lab_key).unwrap(),
                Err(_e) => "error occured when reading from cahce".to_string(),
            }
        }
        Err(_err) => "error occured when writing tot cache".to_string(),
    };

    println!("Ok data {}", value)
}
