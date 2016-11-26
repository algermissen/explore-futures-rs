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

struct Worker<F> {
    u: ForEach<Interval, F>
}


//where F: FnMut() -> Result<(), io::Error>, Self: Sized
impl<F: FnMut((), ) -> Result<(), io::Error>> Worker<F> {
    pub fn new(handle: &Handle, f: F) -> Worker<F>
    //        where F: FnMut((), ) -> Result<(), io::Error>, Self: Sized
    {
        let interval_stream = Interval::new(Duration::new(1, 0), handle).unwrap();

        let x = interval_stream.for_each(f);
        handle.spawn(interval_stream.into_future());
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

    //    let w = Worker < FnMut((), ) -> Result < (), io::Error > >::new( & handle, a);
    let w = Worker::new(&handle, a);


    let interval_stream = Interval::new(Duration::new(10, 0), &handle).unwrap();

    let stream_future = interval_stream.for_each(|_| {
        println!("Hallo");
        Ok(())
    });

    core.run(stream_future).unwrap();
}
