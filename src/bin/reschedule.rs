extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Interval;
use tokio_core::reactor::Handle;
use futures::stream::{Stream};

use std::time::{Instant, Duration};
use futures::Future;
use futures::IntoFuture;
use futures::Then;
use futures::Async;
use futures::Poll;
use std::thread;
use std::cell::Cell;

use tokio_core::reactor::{Core, Timeout};

// A future as return type of getAsynch() method below.
//struct MyFuture {
//    v: i32
//}
//
//impl Future for MyFuture {
//    type Item = i32;
//    type Error = i32;
//
//    fn poll(&mut self) -> Poll<i32, i32> {
//        Ok(Async::Ready(self.v))
//    }
//}


// A struct that holds some state and a handle on
// which to run its worker function.
struct S {
    dur: Duration,
    state: i32,
    h: Handle,
}

impl S {
    // Create a new S and schedule its worker function
    fn new(handle: Handle) -> S {
        let d = Duration::from_millis(5000);
        let s = S { dur: d, state: 42, h: handle };
        s.nextf();
        //        s.f.set(f);
        //        s.h.spawn(f);
        s
    }

    // Schedule next work
    fn nextf(&self) {
        let timeout = Timeout::new(self.dur, &(self.h)).unwrap();
        let t = timeout.then(|r| {
            println!("Worker says hello");
            match r {
                Ok(_) =>
                // Start next work here somehow <====================== *** HOW? ***
                    futures::future::ok::<(), ()>(()),
                Err(_) => futures::future::ok::<(), ()>(()),
            }
        });
        self.h.spawn(t);
    }

    // Will later on be used to access state from other threads in a reactive way.
    //    fn getAsync(&self) -> MyFuture {
    //        MyFuture { v: self.state }
    //    }
}

fn main() {
    // A thread on which background workers will be run.
    let worker_thread = thread::spawn(move || {
        let mut core = Core::new().unwrap();

        // Create a background worker
        let s = S::new(core.handle());

        // An interval stream to drive the background worker futures.
        let interval_stream = Interval::new(Duration::new(1, 0), &(core.handle())).unwrap();
        let stream_future = interval_stream.for_each(|_| {
            println!("Worker driver says hello");
            Ok(())
        });

        core.run(stream_future).unwrap();
    });

    // Request handler threads to be started here (e.g. HTTP server)
    // These threads will eventually use the state the workers work on.

    worker_thread.join().unwrap();
}
