use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub twodns: TwoDNSConfig,
    pub update: UpdateConfig,
}

#[derive(Deserialize)]
pub struct TwoDNSConfig {
    pub url: String,
    pub user: String,
    pub token: String,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

#[derive(Deserialize)]
pub struct UpdateConfig {
    #[serde(default = "default_interval")]
    pub interval: u64,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    #[serde(default = "default_mode")]
    pub mode: ProviderMode,
    pub urls: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Ip {
    pub ip_address: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ProviderMode {
    RoundRobin,
    Random,
}

fn default_timeout() -> u64 {
    5
}

fn default_interval() -> u64 {
    300
}

fn default_mode() -> ProviderMode {
    ProviderMode::Random
}
