use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    // The data shared with the given callback has to be thread-safe.
    // https://doc.rust-lang.org/nomicon/send-and-sync.html
    let counter = Arc::new(Mutex::new(2));
    let cancel_first_option: Arc<Mutex<Option<Box<dyn Fn() + Send + 'static>>>> =
        Arc::new(Mutex::new(None));
    let cancel_first_option_clone = cancel_first_option.clone();

    let on_timeout = move || {
        let mut c = counter.lock().unwrap();
        *c -= 1;
        println!("{}", c);
        if *c == 0 {
            cancel_first_option_clone.lock().unwrap().as_ref().unwrap()();
            println!("The first instance of spawn_timeout has been succesfully stopped");
        }
    };

    // Leaking this inner function to make its lifetime as 'static.
    // https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak
    let static_on_timeout = Box::leak(Box::new(on_timeout));

    let cancel_first = spawn_timeout::spawn_timeout(static_on_timeout, Duration::from_secs(3));
    let _ = spawn_timeout::spawn_timeout(static_on_timeout, Duration::from_secs(2));
    let _ = spawn_timeout::spawn_timeout(static_on_timeout, Duration::from_secs(1));

    *cancel_first_option.lock().unwrap() = Some(cancel_first);

    // Sleeping for a long time for the sake of this example.
    thread::sleep(Duration::from_millis(u64::MAX));
}
