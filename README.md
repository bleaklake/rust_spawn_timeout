# spawn_timeout

Call a subroutine after a constant time interval.

## Examples

### Basic

```rust
use std::{thread, time::Duration};
use spawn_timeout;

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

```

### Inner function

```rust
use std::{thread, time::Duration};
use spawn_timeout;

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

```

### Capturing inner function

```rust
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use spawn_timeout;

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

```
