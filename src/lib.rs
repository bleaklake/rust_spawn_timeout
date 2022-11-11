use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

pub fn spawn_timeout<F>(cb: &'static F, wait_time: Duration) -> Box<dyn Fn() + Send + 'static>
where
    F: Fn() -> () + Sync,
{
    let start = Instant::now();

    let has_stopped = Arc::new(AtomicBool::new(false));
    let has_stopped_cloned = has_stopped.clone();

    thread::spawn(move || {
        let runtime = start.elapsed();

        if let Some(remaining) = wait_time.checked_sub(runtime) {
            thread::sleep(remaining);
        }

        if has_stopped_cloned.load(Ordering::Relaxed) {
            return;
        }

        thread::spawn(cb);
    });

    Box::new(move || {
        has_stopped.store(true, Ordering::Relaxed);
    })
}
