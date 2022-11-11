use std::{thread, time::Duration};

fn on_timeout() {
    println!("timeout!");
}

fn main() {
    let cancel = spawn_timeout::spawn_timeout(&on_timeout, Duration::from_secs(3));
    let _ = spawn_timeout::spawn_timeout(&on_timeout, Duration::from_secs(3));

    // Waiting before cancelling this instance of spawn_timeout.
    thread::sleep(Duration::from_millis(1500));

    cancel();

    println!("The first instance of spawn_timeout has been succesfully stopped");

    // Sleeping for a long time for the sake of this example.
    thread::sleep(Duration::from_millis(u64::MAX));
}
