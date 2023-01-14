use chrono::prelude::*;
use serde_derive::*;

pub use clap::{Parser, Subcommand};

#[derive(PartialEq, Clone, Subcommand, Debug)]
pub enum Commands {
    Test,
    Timer,
    Sid,
    List {
        #[arg(long)]
        sid: Option<String>,
    },
    SwitchOn {
        #[arg(long)]
        sid: Option<String>,
        #[arg(long)]
        ain: String,
    },
    SwitchOff {
        #[arg(long)]
        sid: Option<String>,
        #[arg(long)]
        ain: String,
    },
    Power {
        #[arg(long)]
        sid: Option<String>,
        #[arg(long)]
        ain: String,
    },
    Temperature {
        #[arg(long)]
        sid: Option<String>,
        #[arg(long)]
        ain: String,
    },
    Switchstate {
        #[arg(long)]
        sid: Option<String>,
        #[arg(long)]
        ain: String,
    },
}

#[derive(Clone, Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(long, default_value = "http")]
    pub protocol: String,
    #[arg(long)]
    pub host: Option<String>,

    #[arg(long)]
    pub usr: Option<String>,
    #[arg(long)]
    pub psw: Option<String>,

    #[arg(long)]
    pub sid: Option<String>,
    #[arg(long)]
    pub ain: Option<String>,

    #[arg(long)]
    pub lng: Option<f64>,
    #[arg(long)]
    pub lat: Option<f64>,

    #[arg(long)]
    pub host_mobile: Option<String>,
    #[arg(long)]
    pub port_mobile: Option<u16>,

    #[command(subcommand)]
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
