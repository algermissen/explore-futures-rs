extern crate futures;
extern crate tokio_core;

use std::time::Duration;
use tokio_core::reactor::Interval;
use tokio_core::reactor::Core;
use futures::stream::{Stream};
use futures::stream::ForEach;

use tokio_core::reactor::Remote;
use tokio_core::reactor::Handle;
use std::io;

struct Worker {
    u: i32
}


//where F: FnMut() -> Result<(), io::Error>, Self: Sized
impl Worker {
    pub fn new<F>(handle: &Handle, f: F) -> Worker
        where F: FnMut((), ) -> Result<(), io::Error>, Self: Sized
    {
        let interval_stream = Interval::new(Duration::new(1, 0), handle).unwrap();

        let x = interval_stream.for_each(f);
        Worker { u: x }
    }
}

fn a(e: ()) -> Result<(), io::Error> {
    println!("Ok");
    Ok(())
}


// This starts a 1sec interval stream and prints 'Hallo'
// for each interval.
fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let w = Worker::new(&handle, a);

    //    core.run().unwrap();
}
