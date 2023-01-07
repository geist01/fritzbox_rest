use chrono::prelude::*;
use chrono::Duration;

use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::thread;
use timer::Timer;

use crate::konstanten::*;
use crate::mobileping;
use crate::sunsetrise;

use log::*;

pub fn init_timer(config: &Config) -> Result<()> {
    // main timer once per day, starts ticker

    let config_clone = (*config).clone();

    let child = thread::spawn(move || {
        let timer = Arc::new(Mutex::new(Timer::with_capacity(5)));
        let timer_clone = Arc::clone(&timer);

        let now: DateTime<Utc> = Utc::now() + Duration::seconds(2);
        timer
            .lock()
            .unwrap()
            .schedule(now, Some(Duration::days(1)), move || {
                debug!(target: "fritz", "Tages - Timer gestartet");

                if let Ok(sunsetrise) = sunsetrise::fetch_sunset_intern(&config_clone) {
                    debug!(target: "fritz", "{:?}", sunsetrise);

                    let config_clone2 = config_clone.clone();

                    let timer = timer_clone.lock().unwrap();
                    timer
                        .schedule_with_date(sunsetrise.sunset - Duration::minutes(60), move || {
                            debug!(target: "fritz", "Handler gestartet");
                            let mut aktiv = if let Ok(()) = mobileping::ping_mobile(
                                config_clone2.host_mobile.as_ref().unwrap(),
                                config_clone2.port_mobile.as_ref().unwrap(),
                            ) {
                                debug!(target: "fritz", "Initialer Online Status: on");
                                true
                            } else {
                                debug!(target: "fritz", "Initialer Online Status: off");
                                false
                            };

                            loop {
                                if let Ok(()) = mobileping::ping_mobile(
                                    config_clone2.host_mobile.as_ref().unwrap(),
                                    config_clone2.port_mobile.as_ref().unwrap(),
                                ) {
                                    if !aktiv {
                                        debug!(target: "fritz", "online");
                                        aktiv = true;
                                    }
                                } else if aktiv {
                                    debug!(target: "fritz", "offline");
                                    aktiv = false;
                                }

                                let time = Utc::now().time();
                                if time.hour() >= 23 {
                                    break;
                                }

                                thread::sleep(std::time::Duration::from_millis(10 * 1000));
                            }

                            debug!(target: "fritz", "Handler beendet");
                        })
                        .ignore();
                }
                debug!(target: "fritz", "Tages - Timer beendet");
            })
            .ignore();

        loop {
            thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    let _res = child.join();

    // use tokio::prelude::*;
    // use tokio::timer::Interval;

    // use std::time::{Duration, Instant};

    //     let task = Interval::new(Instant::now(), Duration::from_millis(100))
    // //        .take(10)
    //         .for_each(|instant| {
    //             println!("fire; instant={:?}", instant);
    //             Ok(())
    //         })
    //         .map_err(|e| panic!("interval errored; err={:?}", e));

    //     tokio::run(task);

    Ok(())
}
