use std::{thread, time::Duration};

fn main() {
    let on_timeout = || {
        println!("timeout!");
    };

    // Leaking this inner function to make its lifetime as 'static.
    // https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak
    let static_on_timeout = Box::leak(Box::new(on_timeout));

    let cancel = spawn_timeout::spawn_timeout(static_on_timeout, Duration::from_secs(3));
    let _ = spawn_timeout::spawn_timeout(static_on_timeout, Duration::from_secs(3));

    // Waiting before cancelling this instance of spawn_timeout.
    thread::sleep(Duration::from_millis(1500));

    cancel();

    println!("The first instance of spawn_timeout has been succesfully stopped");

    // Sleeping for a long time for the sake of this example.
    thread::sleep(Duration::from_millis(u64::MAX));
}
