pub use structures::{LaunchAPI, LaunchAPIop, LaunchCache};


use reqwest::Client;
use chrono::{Utc, DateTime, Local};

pub mod structures;

pub async fn update(c: &Client, logs: &mut Vec<(DateTime<Local>, String, u8)>) -> Option<structures::Launch> {
    let req = c.get(crate::constants::LAUNCH_API).send().await;

    return if let Ok(resp) = req {
        let raw_launch: reqwest::Result<structures::LaunchResponse> = resp.json().await;
        if let Ok(launches) = raw_launch {
            if launches.results.is_some() {
                let launch_list = launches.results.unwrap();

                let mut next = launch_list.first().unwrap().clone();
                let previous = crate::utilities::countdown(next.net.clone().unwrap_or(Utc::now().to_string()));

                for launch in launch_list {
                    let time_remaining = crate::utilities::countdown(launch.net.clone().unwrap_or(Utc::now().to_string()));
                    if previous.has_passed {
                        match launch.status.id.clone().unwrap() {
                            1 => {
                                if time_remaining.total_seconds < 30 * 60 {
                                    next = launch;
                                }
                            }
                            x => {
                                match x {
                                    3 | 4 | 6 | 7 => {
                                        if previous.minutes > 20 && time_remaining.total_seconds < 30 * 60 {
                                            next = launch;
                                        } else if previous.minutes > 30 {
                                            next = launch;
                                        } else if time_remaining.total_seconds < 15 * 60 {
                                            next = launch;
                                        } else {
                                            continue;
                                        }
                                    }
                                    _ => {
                                        next = launch;
                                    }
                                }
                            }
                        }
                    } else {
                        continue;
                    }
                };

                Some(next)
            } else {
                if launches.detail.is_some() {
                    logs.push((Local::now(), "Failed to update launch cache".to_string(), 1));
                    logs.push((Local::now(), " ^--> Request throttled by API".to_string(), 1));
                } else {
                    logs.push((Local::now(), "Failed to update launch cache".to_string(), 1));
                    logs.push((Local::now(), " ^--> Unknown error".to_string(), 1));
                }
                None
            }
        } else {
            logs.push((Local::now(), "Failed to update launch cache".to_string(), 1));
            None
        }
    } else {
        logs.push((Local::now(), "Failed to update launch cache".to_string(), 1));
        None
    };
}

pub async fn news_update(c: &Client, logs: &mut Vec<(DateTime<Local>, String, u8)>) -> Option<Vec<structures::Article>> {
    let req = c.get(crate::constants::NEWS_API).send().await;

    return if let Ok(resp) = req {
        let raw_launch: reqwest::Result<Vec<structures::Article>> = resp.json().await;
        if let Ok(launches) = raw_launch {
            Some(launches)
        } else {
            logs.push((Local::now(), "Failed to update news cache".to_string(), 1));
            None
        }
    } else {
        logs.push((Local::now(), "Failed to update news cache".to_string(), 1));
        None
    };
}