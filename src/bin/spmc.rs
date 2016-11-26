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

struct S {
    dur: Duration,
    state: i32,
    h: Handle,
    //    f: Cell<Then<Item = (), Error = ()>>
}

struct MyFuture {
    v: i32
}

impl Future for MyFuture {
    type Item = i32;
    type Error = i32;

    fn poll(&mut self) -> Poll<i32, i32> {
        Ok(Async::Ready(self.v))
    }
}

impl S {
    fn new(handle: Handle) -> S {
        let d = Duration::from_millis(5000);
        let s = S { dur: d, state: 42, h: handle };
        //        let f = s.nextf();
        //        s.f.set(f);
        //        s.h.spawn(f);
        s
    }

    fn nextf(handle: &Handle) {
        let timeout = Timeout::new(self.dur, &(self.h)).unwrap();
        timeout.then(|r| {
            println!("Bang!");
            match r {
                Ok(_) => futures::future::ok::<(), ()>(()),
                Err(_) => futures::future::ok::<(), ()>(()),
            }
        });
    }

    //    fn nextf(&self) {
    //        let timeout = Timeout::new(self.dur, &(self.h)).unwrap();
    //        timeout.then(|r| {
    //            println!("Bang!");
    //            match r {
    //                Ok(_) => futures::future::ok::<(), ()>(()),
    //                Err(_) => futures::future::ok::<(), ()>(()),
    //            }
    //        });
    //    }

    //    fn nextf<A, B, F>(&self) -> Then<A, B, F>
    //        where A: Future,
    //        B: IntoFuture,
    //        F: FnOnce(Result<A::Item, A::Error>) -> B,
    //    {
    //        let timeout = Timeout::new(self.dur, &(self.h)).unwrap();
    //        timeout.then(|r| {
    //            println!("Bang!");
    //            match r {
    //                Ok(_) => futures::future::ok::<(), ()>(()),
    //                Err(_) => futures::future::ok::<(), ()>(()),
    //            }
    //        })
    //    }


    fn getAsync(&self) -> MyFuture {
        MyFuture { v: self.state }
    }
}

fn main() {
    let w = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let s = S::new(core.handle());

        let interval_stream = Interval::new(Duration::new(30, 0), &handle).unwrap();
        let stream_future = interval_stream.for_each(|_| {
            println!("_______________________________ Hello-Worker");
            Ok(())
        });

        core.run(stream_future).unwrap();
    });


    let c1 = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let interval_stream = Interval::new(Duration::new(1, 0), &handle).unwrap();
        let stream_future = interval_stream.for_each(|_| {
            println!("Hello-1");
            Ok(())
        });

        //        handle.spawn(z);

        core.run(stream_future).unwrap();
    });

    let c2 = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let interval_stream = Interval::new(Duration::new(2, 0), &handle).unwrap();
        let stream_future = interval_stream.for_each(|_| {
            println!("Hello-2");
            Ok(())
        });

        //        handle.spawn(z);

        core.run(stream_future).unwrap();
    });

    c1.join().unwrap();


    //    let mut core = Core::new().unwrap();
    //    let handle = core.handle();
    //
    //    let interval_stream = Interval::new(Duration::new(1, 0), &handle).unwrap();
    //    let stream_future = interval_stream.for_each(|_| {
    //        println!("Hallo");
    //        Ok(())
    //    });
    //
    //    let dur = Duration::from_millis(5000);
    //    let timeout = Timeout::new(dur, &handle).unwrap();
    //    let z = timeout.then(|r| {
    //        println!("Bang!");
    //        match r {
    //            Ok(_) => futures::future::ok::<(), ()>(()),
    //            Err(_) => futures::future::ok::<(), ()>(()),
    //        }
    //    });
}
