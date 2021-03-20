use clap::{App, Arg};
use mac_notification_sys::*;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let matches = App::new("yubikey-gpg-watched")
        .version("1.0")
        .author("Pierre BAILLET<pierre@baillet.name>")
        .about("Notifies when yubikey is waiting for a press")
        .arg(
            Arg::new("wait_delay")
                .short('w')
                .long("wait_delay")
                .about("Delay in seconds to wait before firing the notification")
                .default_value("2")
                .takes_value(true),
        )
        .arg(
            Arg::new("notification_title")
                .short('t')
                .about("title to use for the notification")
                .default_value("Your Yubikey is waiting for a press")
                .takes_value(true),
        )
        .arg(
            Arg::new("notification_body")
                .short('b')
                .about("body for the notification")
                .default_value("Press your yubikey, something wants your signature!")
                .takes_value(true),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .about("print debug information verbosely"),
        )
        .get_matches();

    let debug = matches.is_present("debug");

    let bundle = get_bundle_identifier_or_default("onepassword7");
    set_application(&bundle).unwrap();

    let duration_in_seconds: u64 = matches
        .value_of_t("wait_delay")
        .unwrap_or_else(|e| e.exit());
    let notification_title: String = matches
        .value_of_t("notification_title")
        .unwrap_or_else(|e| e.exit());
    let notification_body: String = matches
        .value_of_t("notification_body")
        .unwrap_or_else(|e| e.exit());

    let sleep_duration = Duration::from_secs(duration_in_seconds);
    let mut notified = false;
    loop {
        let mut pid_to_name = HashMap::new();
        let mut gpg_parent_pid: Option<u32> = None;

        for process_info in proclist::iterate_processes_info().filter_map(|r| r.ok()) {
            pid_to_name.insert(process_info.pid, process_info.name.clone());
            if process_info.name == "gpg" {
                gpg_parent_pid = Some(process_info.parent_pid);
            }
        }
        if let Some(parent_pid) = gpg_parent_pid {
            if let Some(name) = pid_to_name.get(&parent_pid) {
                if name == "git" && !notified {
                    if debug {
                        println!("sending notification");
                    }
                    notified = true;
                    send_notification(
                        notification_title.as_str(),
                        &None,
                        notification_body.as_str(),
                        &None,
                    )
                    .unwrap();
                }
            }
        } else {
            notified = false;
        }

        thread::sleep(sleep_duration);
    }
}
