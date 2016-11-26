extern crate futures;
extern crate tokio_core;

use std::time::Duration;
use tokio_core::reactor::Interval;
use tokio_core::reactor::Core;
use futures::stream::{Stream};

// This starts a 1sec interval stream and prints 'Hallo'
// for each interval.
fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let interval_stream = Interval::new(Duration::new(1, 0), &handle).unwrap();

    let stream_future = interval_stream.for_each(|_| {
        println!("Hallo");
        Ok(())
    });

    core.run(stream_future).unwrap();
}
