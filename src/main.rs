use std::fs::File;

use daemonize_me::{Daemon, Group, User};
use std::ffi::OsString;
use std::path::Path;
use users::get_current_username;

const APP_NAME: &str = "mimi-ime";

fn init_dir(local_share_dir: &str) {
    if !Path::new(&local_share_dir).exists() {
        std::fs::create_dir_all(&local_share_dir).expect("Failed to create local share directory");
    }

    if !Path::new(&format!("{}/{}", local_share_dir, APP_NAME)).exists() {
        std::fs::create_dir_all(&format!("{}/{}", local_share_dir, APP_NAME)).expect("Failed to create logs directory");
    }

    if !Path::new(&format!("{}/{}/logs", local_share_dir, APP_NAME)).exists() {
        std::fs::create_dir_all(&format!("{}/{}/logs", local_share_dir, APP_NAME)).expect("Failed to create logs directory");
    }
}

fn main() {
    let username: OsString = get_current_username().expect("Failed to get current username");
    let home_dir = format!("/home/{}", username.to_string_lossy());
    let local_share_dir = format!("{}/.local/share", home_dir);

    init_dir(&local_share_dir);

    let stdout = File::create(format!("{}/{}/logs/{}-info.log", local_share_dir, APP_NAME, APP_NAME)).unwrap();
    let stderr = File::create(format!("{}/{}/logs/{}-error.log", local_share_dir, APP_NAME, APP_NAME)).unwrap();
    let daemon = Daemon::new()
        .pid_file("lotus-ime.pid", Some(false))
        .user(User::try_from(username.to_string_lossy().as_ref()).unwrap())
        .group(Group::try_from("users").unwrap())
        .umask(0o000)
        .work_dir(format!("/home/{}/.local/share/logs/lotus-ime", username.to_string_lossy()))
        .stdout(stdout)
        .stderr(stderr)
        .start();

    match daemon {
        Ok(_) => println!("Daemonized with success"),
        Err(e) => eprintln!("Error, {}", e),
    }

    loop {
        // You wil have to kill this process yourself
    }
}
