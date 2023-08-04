//! The battery widget.

use notify_rust::Notification;
use std::time::Duration;
use tokio::{io, time::sleep};

pub async fn battery() -> io::Result<()> {
    let mut alerted = false;

    loop {
        let parsed = get_battery_capacity()?;
        let class = get_battery_class(parsed);

        println!("(box :class \"bar_item {}\" \"{}%\")", class, parsed);

        if !alerted && parsed <= 10 {
            send_notification();
            alerted = true;
        } else if alerted && parsed > 10 {
            alerted = false;
        }

        sleep(Duration::from_secs(10)).await;
    }
}

fn get_battery_capacity() -> Result<u8, io::Error> {
    let capacity = std::fs::read_to_string("/sys/class/power_supply/BAT1/capacity")?;
    let parsed = capacity.trim().parse::<u8>().unwrap();
    Ok(parsed)
}

fn send_notification() -> notify_rust::NotificationHandle {
    Notification::new()
        .summary("Battery low ")
        .body("Battery is under 10%, please plug in your charger.")
        .urgency(notify_rust::Urgency::Critical)
        .show()
        .unwrap()
}

fn get_battery_class(parsed: u8) -> &'static str {
    match parsed {
        0..=10 => "battery-low",
        11..=30 => "battery-medium",
        31..=99 => "battery-high",
        _ => "battery-full",
    }
}
