use mac_notification_sys::*;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let bundle = get_bundle_identifier_or_default("onepassword7");
    set_application(&bundle).unwrap();

    let one_sec = Duration::from_secs(1);
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
                    println!("sending notification");
                    notified = true;
                    send_notification(
                        "Your Yubikey is waiting for a press",
                        &None,
                        "Press your yubikey, something wants your signature!",
                        &None,
                    )
                    .unwrap();
                }
            }
        } else {
            notified = false;
        }

        thread::sleep(one_sec);
    }
}
