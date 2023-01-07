use crate::konstanten::*;
use crate::errors::*;

use chrono::prelude::*;

pub fn fetch_sunset_intern(config : &Config) -> Result<SunsetRise> {
    let now : DateTime<Utc> = Utc::now();
    let (sunrise, sunset) = sunrise::sunrise_sunset(
        config.lat.unwrap(),
        config.lng.unwrap(),
        now.year(),
        now.month(),
        now.day(),
    );

    // Construct a datetime from epoch:
    let sunset_nd : DateTime<Utc> = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(sunset, 0).unwrap());
    let sunrise_nd : DateTime<Utc> = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(sunrise, 0).unwrap());

    Ok(SunsetRise {
        sunset: sunset_nd,
        sunrise: sunrise_nd,

    })
}
