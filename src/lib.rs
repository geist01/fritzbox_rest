
use crate::konstanten::*;
use anyhow::Result;

pub mod errors;
pub mod konstanten;
pub mod mobileping;
pub mod sunsetrise;
pub mod timertasks;

use reqwest::Client;
use reqwest::Url;

use log::*;

pub fn get_client() -> Client {
    Client::new()
}

pub fn fetch_sid(protocol : &String, host:&Option<String>, user : &Option<String>, psw : &Option<String>, sid : &Option<String>, client: &Client) -> Result<String> {
    use byteorder::{LittleEndian, WriteBytesExt};

    if sid.is_some() {
        return Ok(sid.clone().unwrap());
    }

    let url = format!(
        "{0}://{1}/login_sid.lua",
        protocol,
        host.as_ref().unwrap()
    );

    let challenge_request = client.get(&url).send()?.text()?;
    debug!("body-challenge request = {:?}", challenge_request);

    let session_info: SessionInfo = serde_xml_rs::de::from_str(&challenge_request)?;
    debug!("challenge = {:#?}", session_info);

    let v: Vec<u16> = format!(
        "{0}-{1}",
        session_info.challenge,
        psw.as_ref().unwrap()
    )
    .encode_utf16()
    .collect();

    let mut wtr = vec![];
    for x in v.iter() {
        wtr.write_u16::<LittleEndian>(*x).unwrap();
    }

    let digest = md5::compute(wtr);

    if let Ok(url) = match user {
        None => Url::parse_with_params(
            &url,
            &[(
                "response",
                format!("{0}-{1:#?}", session_info.challenge, digest),
            )],
        ),
        Some(user) => Url::parse_with_params(
            &url,
            &[
                (
                    "response",
                    format!("{0}-{1:#?}", session_info.challenge, digest),
                ),
                ("username", user.to_string()),
            ],
        ),
    } {
        let sid_request = client.get(url).send()?.text()?;
        debug!("body-sid request = {:?}", challenge_request);

        let session_info: SessionInfo = serde_xml_rs::de::from_str(&sid_request).unwrap();
        debug!("sid = {}", session_info.sid);

        return Ok(session_info.sid);
    }

    Ok("ddd".to_string())
}

pub fn logout_sid(client: &Client, protocol : &String, host : &Option<String>, sid: &str) -> Result<()> {
    let url = Url::parse_with_params(
        &format!(
            "{0}://{1}/login_sid.lua",
            protocol,
            host.as_ref().unwrap()
        ),
        &[("sid", sid), ("logout", "1")],
    )?;

    let res = client.get(url).send()?.text()?;
    debug!("body-devicelist result = {:?}", res);

    Ok(())
}

pub fn list_devices(client: &Client, protocol : &String, host : &Option<String>,  sid: &str) -> Result<DeviceListInfos> {
    let url = Url::parse_with_params(
        &format!(
            "{0}://{1}/webservices/homeautoswitch.lua",
            protocol,
            host.as_ref().unwrap()
        ),
        &[("sid", sid), ("switchcmd", "getdevicelistinfos")],
    )?;

    let res = client.get(url).send()?.text()?;
    debug!("body-devicelist result = {:?}", res);

    let devices_list: DeviceListInfos = serde_xml_rs::de::from_str(&res)?;
    debug!("devices = {:#?}", devices_list);

    for x in &devices_list.device {
        debug!(target : "fritz", "Device: {0}, Identifier/AIN: {1}", x.productname, x.identifier);
    }

    Ok(devices_list)
}

pub fn read_switchstate(client: &Client, protocol : &String, host : &Option<String>, sid: &str, ain : &str) -> Result<bool> {
    let url = Url::parse_with_params(
        &format!(
            "{0}://{1}/webservices/homeautoswitch.lua",
            protocol,
            host.as_ref().unwrap()
        ),
        &[
            ("sid", sid),
            ("ain", ain),
            ("switchcmd", "getswitchstate"),
        ],
    )?;

    let res = client.get(url).send()?.text()?.replace('\n', "");
    debug!(target : "fritz", "swichtstate result = {:?}", res);

    let b = res.parse::<i32>()?;
    if b == 1 {
        return Ok(true);
    }

    Ok(false)
}

pub fn read_temperature(client: &Client, protocol : &String, host : &Option<String>, sid: &str, ain : &str) -> Result<usize> {
    let url = Url::parse_with_params(
        &format!(
            "{0}://{1}/webservices/homeautoswitch.lua",
            protocol,
            host.as_ref().unwrap()
        ),
        &[
            ("sid", sid),
            ("ain", ain),
            ("switchcmd", "gettemperature"),
        ],
    )?;

    let res = client.get(url).send()?.text()?.replace('\n', "");
    debug!(target : "fritz", "body-temperature result = {:?}", res);

    if let Ok(temp) = res.parse::<usize>() {
        return Ok(temp);
    }

    Ok(0)
}

pub fn read_consumption(client: &Client, protocol : &String, host : &Option<String>, sid: &str, ain : &str) -> Result<f64> {
    let url = Url::parse_with_params(
        &format!(
            "{0}://{1}/webservices/homeautoswitch.lua",
            protocol,
            host.as_ref().unwrap()
        ),
        &[
            ("sid", sid),
            ("ain", ain),
            ("switchcmd", "getswitchpower"),
        ],
    )?;

    let res = client.get(url).send()?.text()?.replace('\n', "");
    debug!("body-power result = {:?}", res);

    if let Ok(power) = res.parse::<f64>() {
        return Ok(power);
    }

    Ok(0.0)
}

pub fn set_switch_on_off(client: &Client, protocol : &String, host : &Option<String>, sid: &str, ain : &str, on: bool) -> Result<()> {
    let command = if on { "setswitchon" } else { "setswitchoff" };

    let url = Url::parse_with_params(
        &format!(
            "{0}://{1}/webservices/homeautoswitch.lua",
            protocol,
            host.as_ref().unwrap()
        ),
        &[
            ("sid", sid),
            ("ain", ain),
            ("switchcmd", command),
        ],
    )?;

    let res = client.get(url).send()?.text()?;
    debug!("body-power result = {:?}", res);

    Ok(())
}

