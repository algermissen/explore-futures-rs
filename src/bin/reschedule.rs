extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Interval;
use tokio_core::reactor::Handle;
use futures::stream::{Stream};

use std::time::{Instant, Duration};
use futures::Future;
use futures::IntoFuture;
use futures::Then;
use futures::Map;
use futures::Async;
use futures::future::Ok;
use futures::Poll;
use std::thread;
use std::cell::Cell;

use tokio_core::reactor::{Core, Timeout};

struct Error {
    code: i32
}

impl Error {
    fn new(c: i32) -> Error {
        Error { code: c }
    }
}

// A future as return type of getAsynch() method below.
//struct After {
//    d: Duration,
//    s: State
//}
//
//impl Future for After {
//    type Item = State;
//    type Error = Error;
//
//    fn poll(&mut self) -> Poll<State, Error> {
//        Ok(Async::Ready(self.s))
//    }
//}

enum Cmd<S: Clone> {
    Start(S),
    //    Immediately(Duration, S),
    After(Duration, S)
}


// A struct that holds some state and a handle on
// which to run its worker function.
struct Actor<S: Clone> {
    dur: Duration,
    state: S,
    //fu: fn(S) -> Future<Item = S, Error = Error>,
    fu: fn(S) -> Cmd<S>,
}

impl<S: 'static + Clone> Actor<S> {
    // Create a new S and schedule its worker function
    //    fn new(handle: Handle, start: S, f: fn(S) -> Future<Item = S, Error = Error>) -> Actor<S> {
    fn new(handle: Handle, start: S, f: fn(S) -> Cmd<S>) -> Actor<S> {
        let d = Duration::from_millis(5000);
        let a: Actor<S> = Actor { dur: d, state: start.clone(), fu: f };
        let cmd = Cmd::Start(start.clone());
        Actor::nextf2(handle, cmd, f);
        //handle.spawn(fut);
        a
    }

    fn nextf2(handle: Handle, cmd: Cmd<S>, f: fn(S) -> Cmd<S>) {
        let futu = match cmd {
            Cmd::Start(s) => {
                let timeout = Timeout::new(Duration::from_millis(5000), &handle).unwrap();
                let x = handle.clone();
                let s2 = s.clone();
                let f2 = f.clone();
                let futu = timeout.then(move |r| {
                    println!("Worker says start");
                    match r {
                        Ok(_) => {
                            let cmd2 = f2(s2);
                            Actor::nextf2(x, cmd2, f);
                            futures::future::ok::<(), ()>(())
                        },
                        Err(_) => futures::future::ok::<(), ()>(()),
                    }
                });
                handle.spawn(futu);
            },
            Cmd::After(d, s) => {
                let timeout = Timeout::new(d, &handle).unwrap();
                let x = handle.clone();
                let s2 = s.clone();
                let f2 = f.clone();
                let futu = timeout.then(move |r| {
                    println!("Worker says after");
                    match r {
                        Ok(_) => {
                            let cmd2 = f2(s2);
                            Actor::nextf2(x, cmd2, f);
                            futures::future::ok::<(), ()>(())
                        },
                        Err(_) => futures::future::ok::<(), ()>(()),
                    }
                });
                handle.spawn(futu);
            },
            //            Cmd::Run(rf) => {
            //                let timeout = Timeout::new(d, &handle).unwrap();
            //                let x = handle.clone();
            //
            //                let s2 = s.clone();
            //                let f2 = f.clone();
            //                let futu = rf.then(move |r| {
            //                    println!("Worker says run");
            //                    match r {
            //                        Ok(_) => {
            //                            let cmd2 = f2(s2);
            //                            Actor::nextf2(x, cmd2, f);
            //                            futures::future::ok::<(), ()>(())
            //                        },
            //                        Err(_) => futures::future::ok::<(), ()>(()),
            //                    }
            //                });
            //                handle.spawn(futu);
            //            }
        };
    }

    // Will later on be used to access state from other threads in a reactive way.
    //    fn getAsync(&self) -> MyFuture {
    //        MyFuture { v: self.state }
    //    }
}

#[derive(Copy, Clone)]
enum State {
    Start(i32),
    Foo,
    End
}


//fn testGetFromUpstream<R, F>(handle: &Handle, d: Duration, v: R) -> Map<Timeout, F>
//    where R: Clone, F: FnOnce(()) -> R,
//{
//    let timeout = Timeout::new(d, handle).unwrap();
//    let futu = timeout.map(|r| { v });
//    futu
//}

fn testSimulateRemoteGetValue<R, F>(handle: &Handle, d: Duration, v: R) -> Map<Timeout, F>
    where R: Clone, F: FnOnce(()) -> R,
{
    let timeout = Timeout::new(d, handle).unwrap();
    let fut: Map<Timeout, F> = timeout.map(|r| -> R { v });
    fut
}

//fn map<F, U>( self , f: F) -> Map < Self, F>
//where F: FnOnce( Self::Item) -> U,
//Self: Sized,
//{
//assert_future::< U, Self::Error, _> (map::new( self, f))
//}

//type FOO = fn(()) -> i32;
//
//fn testGetFromUpstream(handle: &Handle, d: Duration, v: i32) -> Map<Timeout, fn(()) -> i32> {
//    let timeout = Timeout::new(d, handle).unwrap();
//    let c: fn(()) -> i32 = |r| { v };
//    timeout.map(c)
//}

fn main() {
    // A thread on which background workers will be run.
    let worker_thread = thread::spawn(move || {
        let mut core = Core::new().unwrap();

        //        fn add_10<F>(f: F) -> Map<F, fn(i32) -> i32>
        //            where F: Future<Item = i32>,
        //        {
        //            fn do_map(i: i32) -> i32 { i + 10 }
        //            f.map(do_map)
        //        }
        //
        //        fn doIt<S>(s:S) -> Map<F, fn(i32) -> i32>
        //            where F: Future<Item = S>,
        //        {
        //
        //        }

        //        fn update(state: State) -> Future<Item = State, Error = Error> {
        fn update(state: State) -> Cmd<State> {
            match state {
                State::Start(s) => {
                    println!("start");
                    Cmd::After(Duration::new(1, 0), State::Foo)
                },
                //                State::Foo => {
                //                    println!("foo");
                //                    Cmd::Run(|| {
                //
                //                    })
                //                },
                State::End => {
                    println!("end");
                    Cmd::After(Duration::new(1, 0), State::End)
                },
            }
        }

        // Create a background worker
        let a: Actor<State> = Actor::new(core.handle(), State::Start(10), update);

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
