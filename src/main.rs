use anyhow::Result;
use fritzbox_rest::konstanten::*;

use log::*;

fn read_config() -> Result<Config> {
    use ini::Ini;

    let mut config = Config::parse();

    if let Ok(ini) = Ini::load_from_file("config/defaults.ini") {
        let geo = ini.section(Some("geo".to_owned())).unwrap();
        config.lng.get_or_insert(geo.get("lng").unwrap().parse::<f64>()?);
        config.lat.get_or_insert(geo.get("lat").unwrap().parse::<f64>()?);

        let server = ini.section(Some("server".to_owned())).unwrap();
        config.host.get_or_insert(server.get("host").unwrap().to_string());
        config.usr.get_or_insert(server.get("user").unwrap().to_string());
        config.psw.get_or_insert(server.get("psw").unwrap().to_string());

        let dev = ini.section(Some("device".to_owned())).unwrap();
	config.host_mobile.get_or_insert(dev.get("host_mobile").unwrap().to_string());
	config.port_mobile.get_or_insert(dev.get("port_mobile").unwrap().parse::<u16>()?);
    };

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
                config.port_mobile.unwrap(),
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
            let sid = fritzbox_rest::fetch_sid(&config.protocol, &config.host, &config.usr, &config.psw, &Option::None, &client)?;
            debug!(target : "fritz", "OK: sid = {}", sid);
        }
        Commands::List { ref sid } => {
            let sid = fritzbox_rest::fetch_sid(&config.protocol, &config.host, &config.usr, &config.psw, sid, &client)?;
            fritzbox_rest::list_devices(&client, &config.protocol, &config.host, &sid)?;
            if config.sid.is_none() {
                fritzbox_rest::logout_sid(&client, &config.protocol, &config.host, &sid)?;
            }
        }
        Commands::Temperature { ref sid, ref ain } => {
            let sid = fritzbox_rest::fetch_sid(&config.protocol, &config.host, &config.usr, &config.psw, sid, &client)?;
	    let x = fritzbox_rest::read_temperature(&client, &config.protocol, &config.host, &sid, ain)?;
            debug!(target : "fritz", "Wert {}", x);
            if config.sid.is_none() {
                fritzbox_rest::logout_sid(&client, &config.protocol, &config.host, &sid)?;
            }
        }
        Commands::Power { ref sid, ref ain } => {
            let sid = fritzbox_rest::fetch_sid(&config.protocol, &config.host, &config.usr, &config.psw, sid, &client)?;
            let x = fritzbox_rest::read_consumption(&client, &config.protocol, &config.host, &sid, ain)?;
            debug!(target : "fritz", "Wert {}", x);
            if config.sid.is_none() {
                fritzbox_rest::logout_sid(&client, &config.protocol, &config.host, &sid)?;
            }
        }
        Commands::SwitchOn { ref sid, ref ain } => {
            let sid = fritzbox_rest::fetch_sid(&config.protocol, &config.host, &config.usr, &config.psw, sid, &client)?;
            fritzbox_rest::set_switch_on_off(&client, &config.protocol, &config.host, &sid, ain, true)?;
            if config.sid.is_none() {
                fritzbox_rest::logout_sid(&client, &config.protocol, &config.host, &sid)?;
            }
        }
        Commands::SwitchOff { ref sid, ref ain } => {
            let sid = fritzbox_rest::fetch_sid(&config.protocol, &config.host, &config.usr, &config.psw, sid, &client)?;
            fritzbox_rest::set_switch_on_off(&client, &config.protocol, &config.host, &sid, ain, false)?;
            if config.sid.is_none() {
                fritzbox_rest::logout_sid(&client, &config.protocol, &config.host, &sid)?;
            }
        }
        Commands::Switchstate { ref sid, ref ain } => {
            let sid = fritzbox_rest::fetch_sid(&config.protocol, &config.host, &config.usr, &config.psw, sid, &client)?;
	    let x = fritzbox_rest::read_switchstate(&client, &config.protocol, &config.host, &sid, ain)?;
            info!(target : "fritz", "Wert {}", x);
            if config.sid.is_none() {
                fritzbox_rest::logout_sid(&client, &config.protocol, &config.host, &sid)?;
            }
        }
    }

    Ok(())
}

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    if let Err(ref e) = run() {
	println!("{:?}", e);
        ::std::process::exit(1);
    }
}
