use chrono::prelude::*;
use serde_derive::*;

#[derive(PartialEq, Clone)]
pub enum Commands {
    Test,
    Timer,
    Sid,
    List,
    SwitchOn,
    SwitchOff,
    GetPower,
    GetTemperature,
    GetState,
}

#[derive(Clone)]
pub struct Config {
    pub protocol: String,
    pub host: String,
    pub usr: Option<String>,
    pub psw: String,
    pub sid: Option<String>,
    pub ain: Option<String>,

    pub lng: Option<f64>,
    pub lat: Option<f64>,

    pub host_mobile: Option<String>,
    pub port_mobile: Option<String>,

    // Commands
    pub command: Commands,
}

#[derive(Debug, Deserialize)]
pub struct SessionInfo {
    #[serde(rename = "SID")]
    pub sid: String,

    #[serde(rename = "Challenge")]
    pub challenge: String,
}

#[derive(Debug, Deserialize)]
pub struct DeviceListInfos {
    pub device: Vec<DeviceListeInfosItem>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceListeInfosItem {
    pub identifier: String,
    pub productname: String,
}

#[derive(Debug)]
pub struct SunsetRise {
    pub sunset: DateTime<Utc>,
    pub sunrise: DateTime<Utc>,
}
