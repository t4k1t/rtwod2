mod config;
mod errors;

use config::{Config, Ip, ProviderMode, TwoDNSConfig, UpdateConfig};
use errors::*;

use clap::{App, Arg};
use env_logger;
use env_logger::Env;
use log::*;
use rand::Rng;
use std::collections::HashMap;
use std::iter::Cycle;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    let matches = App::new("rtwod2")
        .version("0.1")
        .author("Thomas Kager<thomas@monochromatic.cc>")
        .about("TwoDNS client")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("load configuration from FILE")
                .takes_value(true),
        )
        .get_matches();

    // Default to "info" log level
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let default_config_path = xdg_config_home().join("rtwod2/config.toml");
    let config_path = matches
        .value_of("config")
        .unwrap_or_else(|| default_config_path.to_str().unwrap());
    debug!("config path arg: {}", config_path);

    let config: Config = toml::from_str(&std::fs::read_to_string(config_path).unwrap()).unwrap();
    let mut update_urls = config.update.urls.iter().cycle();
    let pause = Duration::from_secs(config.update.interval);

    // Main loop
    let mut first_iteration = true;
    loop {
        if !first_iteration {
            thread::sleep(pause);
        } else {
            first_iteration = false;
        }
        let fetched_ip = match fetch_ip(&config.update, &mut update_urls) {
            Ok(ip) => ip,
            Err(err) => {
                error!("failed to fetch external IP: {}", err);
                continue;
            }
        };
        debug!("external IP: {}", fetched_ip.ip_address);

        let recorded_ip = match fetch_recorded_ip(&config.twodns) {
            Ok(ip) => ip,
            Err(err) => {
                error!("failed to fetch recorded IP: {}", err);
                continue;
            }
        };
        debug!("recorded IP: {}", recorded_ip.ip_address);

        if fetched_ip.ip_address != recorded_ip.ip_address {
            let updated_ip = match update_recorded_ip(&config.twodns, &fetched_ip.ip_address) {
                Ok(ip) => ip,
                Err(err) => {
                    error!("failed to update recorded IP: {}", err);
                    continue;
                }
            };
            info!("updated recorded IP: {}", updated_ip);
        }
    }
}

// Taken from i3status-rust
pub fn xdg_config_home() -> PathBuf {
    // In the unlikely event that $HOME is not set, it doesn't really matter
    // what we fall back on, so use /.config.
    let config_path = std::env::var("XDG_CONFIG_HOME").unwrap_or(format!(
        "{}/.config",
        std::env::var("HOME").unwrap_or_else(|_| "".to_string())
    ));
    PathBuf::from(&config_path)
}

fn update_recorded_ip(twodns: &TwoDNSConfig, ip: &str) -> Result<String, UpdateError> {
    // TODO: Cache recorded ip
    let client = reqwest::blocking::Client::new();
    debug!("updating recorded IP...");
    let mut payload = HashMap::new();
    payload.insert("ip_address", &ip);
    client
        .put(&twodns.url)
        .timeout(Duration::new(twodns.timeout, 0))
        .basic_auth(&twodns.user, Some(&twodns.token))
        .json(&payload)
        .send()?
        .error_for_status()?;
    Ok(String::from(ip))
}

fn fetch_ip(
    update: &UpdateConfig,
    update_urls: &mut Cycle<std::slice::Iter<String>>,
) -> Result<Ip, FetchError> {
    debug!("fetching external IP...");
    let mode = &update.mode;
    let url = match mode {
        // update_urls iterator is non-exhaustive so next() will always return something
        ProviderMode::RoundRobin => update_urls.next().unwrap(),
        ProviderMode::Random => {
            let mut rng = rand::thread_rng();
            let pos = rng.gen_range(0, update.urls.len() - 1);
            update_urls.nth(pos).unwrap()
        }
    };
    debug!("from URL: {}", url);

    let client = reqwest::blocking::Client::new();
    let body = client
        .get(url)
        .timeout(Duration::new(update.timeout, 0))
        .send()?
        .error_for_status()?
        .text()?;
    debug!("response body: {:?}", body);
    let ip = Ip {
        ip_address: String::from(body.trim()),
    };
    Ok(ip)
}

fn fetch_recorded_ip(twodns: &TwoDNSConfig) -> Result<Ip, FetchError> {
    let client = reqwest::blocking::Client::new();
    debug!("fetching recorded IP...");
    let ip = client
        .get(&twodns.url)
        .timeout(Duration::new(twodns.timeout, 0))
        .basic_auth(&twodns.user, Some(&twodns.token))
        .send()?
        .error_for_status()?
        .json()?;
    Ok(ip)
}
