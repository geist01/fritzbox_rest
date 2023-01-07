use anyhow::Result;
use fritzbox_rest::errors::*;
use fritzbox_rest::konstanten::*;

use log::*;

fn read_config() -> Result<Config> {
    use clap::{App, Arg, SubCommand};
    use ini::Ini;

    let matches = App::new("AVM Fritzbox API")
        .version("1.0")
        .author("IB")
        .subcommand(SubCommand::with_name("test"))
        .subcommand(SubCommand::with_name("timer"))
        .subcommand(SubCommand::with_name("sid"))
        .subcommand(
            SubCommand::with_name("list").arg(
                Arg::with_name("sid")
                    .takes_value(true)
                    .short("s")
                    .long("sid")
                    .value_name("SID"),
            ),
        )
        .subcommand(
            SubCommand::with_name("power")
                .arg(
                    Arg::with_name("sid")
                        .takes_value(true)
                        .short("s")
                        .long("sid")
                        .value_name("SID"),
                )
                .arg(
                    Arg::with_name("ain")
                        .required(true)
                        .takes_value(true)
                        .short("a")
                        .long("ain")
                        .value_name("AIN"),
                ),
        )
        .subcommand(
            SubCommand::with_name("temperature")
                .arg(
                    Arg::with_name("sid")
                        .takes_value(true)
                        .short("s")
                        .long("sid")
                        .value_name("SID"),
                )
                .arg(
                    Arg::with_name("ain")
                        .required(true)
                        .takes_value(true)
                        .short("a")
                        .long("ain")
                        .value_name("AIN"),
                ),
        )
        .subcommand(
            SubCommand::with_name("switchon")
                .arg(
                    Arg::with_name("sid")
                        .takes_value(true)
                        .short("s")
                        .long("sid")
                        .value_name("SID"),
                )
                .arg(
                    Arg::with_name("ain")
                        .required(true)
                        .takes_value(true)
                        .short("a")
                        .long("ain")
                        .value_name("AIN"),
                ),
        )
        .subcommand(
            SubCommand::with_name("switchoff")
                .arg(
                    Arg::with_name("sid")
                        .takes_value(true)
                        .short("s")
                        .long("sid")
                        .value_name("SID"),
                )
                .arg(
                    Arg::with_name("ain")
                        .required(true)
                        .takes_value(true)
                        .short("a")
                        .long("ain")
                        .value_name("AIN"),
                ),
        )
        .subcommand(
            SubCommand::with_name("switchstate")
                .arg(
                    Arg::with_name("sid")
                        .takes_value(true)
                        .short("s")
                        .long("sid")
                        .value_name("SID"),
                )
                .arg(
                    Arg::with_name("ain")
                        .required(true)
                        .takes_value(true)
                        .short("a")
                        .long("ain")
                        .value_name("AIN"),
                ),
        )
        .get_matches();

    let mut config = match Ini::load_from_file("config/defaults.ini") {
        Ok(ini) => {
            let geo = ini.section(Some("geo".to_owned())).unwrap();
            let dev = ini.section(Some("device".to_owned())).unwrap();
            let server = ini.section(Some("server".to_owned())).unwrap();

            Config {
                protocol: String::from("http"),
                host: server.get("host").unwrap().to_string(),
                usr: server.get("user").map(|s| s.to_owned()),
                psw: server.get("psw").unwrap().to_string(),
                lat: geo.get("lat").and_then(|s| s.parse::<f64>().ok()),
                lng: geo.get("lng").and_then(|s| s.parse::<f64>().ok()),
                host_mobile: dev.get("host_mobile").map(|s| s.to_owned()),
                port_mobile: dev.get("port_mobile").map(|s| s.to_owned()),
                sid: None,
                ain: None,
                command: Commands::List,
            }
        }
        Err(_) => {
            return Err(
                FritzboxError::MissingParameter("host / psw are mandatory".to_string()).into(),
            );
        }
    };

    match matches.subcommand() {
        ("test", Some(m)) => {
            config.command = Commands::Test;
            config.lng = m
                .value_of("lng")
                .map(|s| s.parse::<f64>().unwrap())
                .or(config.lng);
            config.lat = m
                .value_of("lat")
                .map(|s| s.parse::<f64>().unwrap())
                .or(config.lat);
            config.host_mobile = m
                .value_of("host_mobile")
                .map(|s| s.to_string())
                .or(config.host_mobile);
            config.port_mobile = m
                .value_of("port_mobile")
                .map(|s| s.to_string())
                .or(config.port_mobile);
        }
        ("timer", Some(m)) => {
            config.command = Commands::Timer;
            config.lng = m
                .value_of("lng")
                .map(|s| s.parse::<f64>().unwrap())
                .or(config.lng);
            config.lat = m
                .value_of("lat")
                .map(|s| s.parse::<f64>().unwrap())
                .or(config.lat);
            config.host_mobile = m
                .value_of("host_mobile")
                .map(|s| s.to_string())
                .or(config.host_mobile);
            config.port_mobile = m
                .value_of("port_mobile")
                .map(|s| s.to_string())
                .or(config.port_mobile);

            config.sid = m.value_of("sid").map(|s| s.to_string());
            config.ain = m.value_of("ain").map(|s| s.to_string());
        }
        ("sid", Some(_)) => {
            config.command = Commands::Sid;
            config.sid = None;
            config.ain = None;
        }
        ("power", Some(m)) => {
            config.command = Commands::GetPower;
            config.sid = m.value_of("sid").map(|s| s.to_string());
            config.ain = m.value_of("ain").map(|s| s.to_string());
        }
        ("temperature", Some(m)) => {
            config.command = Commands::GetTemperature;
            config.sid = m.value_of("sid").map(|s| s.to_string());
            config.ain = m.value_of("ain").map(|s| s.to_string());
        }
        ("switchon", Some(m)) => {
            config.command = Commands::SwitchOn;
            config.sid = m.value_of("sid").map(|s| s.to_string());
            config.ain = m.value_of("ain").map(|s| s.to_string());
        }
        ("switchoff", Some(m)) => {
            config.command = Commands::SwitchOff;
            config.sid = m.value_of("sid").map(|s| s.to_string());
            config.ain = m.value_of("ain").map(|s| s.to_string());
        }
        ("switchstate", Some(m)) => {
            config.command = Commands::GetState;
            config.sid = m.value_of("sid").map(|s| s.to_string());
            config.ain = m.value_of("ain").map(|s| s.to_string());
        }
        _ => {
            config.sid = None;
            config.ain = None;
        }
    }

    Ok(config)
}

fn run() -> Result<()> {
    let config = read_config()?;

    let client = fritzbox_rest::get_client();

    match config.command {
        Commands::Test => {
            let x = fritzbox_rest::sunsetrise::fetch_sunset_intern(&config)?;
            debug!(target : "fritz", "Sunset {:?}", x);

            if let Ok(()) = fritzbox_rest::mobileping::ping_mobile(
                &config.host_mobile.unwrap(),
                &config.port_mobile.unwrap(),
            ) {
                debug!(target : "fritz", "Mobile online");
            }
        }
        Commands::Timer => {
            if fritzbox_rest::timertasks::init_timer(&config).is_err() {
                error!(target: "fritz", "Timer could not be started");
            }
        }
        Commands::Sid => {
            let sid = fritzbox_rest::fetch_sid(&config, &client)?;
            debug!(target : "fritz", "OK: sid = {}", sid);
        }
        Commands::List => {
            let sid = fritzbox_rest::fetch_sid(&config, &client)?;
            fritzbox_rest::list_devices(&client, &config, &sid)?;
            if config.sid.is_none() {
                fritzbox_rest::logout_sid(&client, &config, &sid)?;
            }
        }
        Commands::GetTemperature => {
            let sid = fritzbox_rest::fetch_sid(&config, &client)?;
            let x = fritzbox_rest::read_temperature(&client, &config, &sid)?;
            debug!(target : "fritz", "Wert {}", x);
            if config.sid.is_none() {
                fritzbox_rest::logout_sid(&client, &config, &sid)?;
            }
        }
        Commands::GetPower => {
            let sid = fritzbox_rest::fetch_sid(&config, &client)?;
            let x = fritzbox_rest::read_consumption(&client, &config, &sid)?;
            debug!(target : "fritz", "Wert {}", x);
            if config.sid.is_none() {
                fritzbox_rest::logout_sid(&client, &config, &sid)?;
            }
        }
        Commands::SwitchOn => {
            let sid = fritzbox_rest::fetch_sid(&config, &client)?;
            fritzbox_rest::set_switch_on_off(&client, &config, &sid, true)?;
            if config.sid.is_none() {
                fritzbox_rest::logout_sid(&client, &config, &sid)?;
            }
        }
        Commands::SwitchOff => {
            let sid = fritzbox_rest::fetch_sid(&config, &client)?;
            fritzbox_rest::set_switch_on_off(&client, &config, &sid, false)?;
            if config.sid.is_none() {
                fritzbox_rest::logout_sid(&client, &config, &sid)?;
            }
        }
        Commands::GetState => {
            let sid = fritzbox_rest::fetch_sid(&config, &client)?;
            let x = fritzbox_rest::read_switchstate(&client, &config, &sid)?;
            info!(target : "fritz", "Wert {}", x);
            if config.sid.is_none() {
                fritzbox_rest::logout_sid(&client, &config, &sid)?;
            }
        }
    }

    Ok(())
}

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    if let Err(ref _e) = run() {
        ::std::process::exit(1);
    }
}
