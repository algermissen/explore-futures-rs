extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Interval;
use futures::stream::{Stream};

use std::time::{Instant, Duration};
use futures::Future;

use tokio_core::reactor::{Core, Timeout};

// This starts a 1sec interval stream that prints 'Hallo'
// for each interval and it starts a single timeout future
// by spawning it on the core loop driven by the interval.
fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let interval_stream = Interval::new(Duration::new(1, 0), &handle).unwrap();
    let stream_future = interval_stream.for_each(|_| {
        println!("Hallo");
        Ok(())
    });

    let dur = Duration::from_millis(5000);
    let timeout = Timeout::new(dur, &handle).unwrap();
    let z = timeout.then(|r| {
        println!("Bang!");
        match r {
            Ok(_) => futures::future::ok::<(), ()>(()),
            Err(_) => futures::future::ok::<(), ()>(()),
        }
    });

    handle.spawn(z);

    core.run(stream_future).unwrap();
}
