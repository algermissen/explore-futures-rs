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
use std::result::Result;
use std::io;
use futures::Async;
use futures::future::Ok;
use futures::Poll;
use std::thread;
use std::cell::Cell;
use std::boxed::Box;
//use std::boxed::FnBox;

use tokio_core::reactor::{Core, Timeout};

#[derive(Debug)]
struct Error {
    code: i32
}

impl Error {
    fn new(c: i32) -> Error {
        Error { code: c }
    }
}


//struct MyFuture<S: Clone> {
//    v: S
//}
//
//impl<S: Clone> Future for MyFuture<S>
//{
//    type Item = S;
//    type Error = Error;
//
//    fn poll(&mut self) -> Poll<S, Error> {
//        //self.get().poll()
//        Ok(Async::Ready(self.v.clone()))
//    }
//}

//impl<S: Clone> IntoFuture for MyFuture<S> {
//    type Future = MyFuture<S>;
//    type Item = S;
//    type Error = Error;
//
//    fn into_future(self) -> MyFuture<S> {
//        self
//    }
//}

//impl<F, R> Future for Lazy<F, R>
//where F: FnOnce() -> R,
//R: IntoFuture,


enum Cmd<S: Clone> {
    Start(S),
    //    Run(Box<FnBox(Handle) -> MyFuture<S>>),
    //Run(Box<FnOnce(Handle) -> MyFuture<S>>),
    Run(fn(handle: Handle) -> Box<Future<Item = S, Error = ()>>),
    After(Duration, S)
}


// A struct that holds some state and a handle on
// which to run its worker function.
#[derive(Debug)]
struct Actor<S: Clone> {
    //dur: Duration,
    state: S,
    //fu: fn(S) -> Cmd<S>,
}

impl<S: 'static + Clone> Actor<S> {
    fn new(handle: Handle, start_state: S, guts: fn(S) -> Cmd<S>) -> Actor<S> {
        //        let d = Duration::from_millis(5000);
        //        let a: Actor<S> = Actor { dur: d, state: start.clone(), fu: f };
        let a: Actor<S> = Actor { state: start_state.clone() };
        let cmd = Cmd::Start(start_state.clone());
        Actor::run_guts(handle, cmd, guts);
        a
    }

    fn run_guts(handle: Handle, cmd: Cmd<S>, guts: fn(S) -> Cmd<S>) {
        let futu = match cmd {
            Cmd::Start(state) => {
                let timeout = Timeout::new(Duration::from_millis(5000), &handle).unwrap();
                let handle_copy = handle.clone();
                let state_copy = state.clone();
                let guts_copy = guts.clone();
                let fut = timeout.then(move |r| {
                    println!("******************************** START **********");
                    let next_cmd = guts_copy(state_copy);
                    Actor::run_guts(handle_copy, next_cmd, guts_copy);
                    // TODO match on r
                    futures::future::ok::<(), ()>(())
                });
                handle.spawn(fut);
            },
            Cmd::After(d, state) => {
                let timeout = Timeout::new(d, &handle).unwrap();
                let handle_copy = handle.clone();
                let state_copy = state.clone();
                let guts_copy = guts.clone();
                let fut = timeout.then(move |r| {
                    println!("******************************** AFTER **********");
                    let next_cmd = guts_copy(state_copy);
                    Actor::run_guts(handle_copy, next_cmd, guts_copy);
                    // TODO match on r
                    futures::future::ok::<(), ()>(())
                });
                handle.spawn(fut);
            },
            Cmd::Run(work_future) => {
                let handle_copy = handle.clone();
                let handle_copy2 = handle.clone();
                //                let state_copy = state.clone();
                let guts_copy = guts.clone();
                let fut0: Box<Future<Item = S, Error = ()>> = work_future(handle_copy);
                let fut = fut0.then(move |x: Result<S, ()>| {
                    let new_state = x.unwrap();
                    println!("******************************** RUN **********");
                    let next_cmd = guts_copy(new_state);
                    Actor::run_guts(handle_copy2, next_cmd, guts_copy);
                    // TODO match on r
                    futures::future::ok::<(), ()>(())
                });
                handle.spawn(fut);
            }
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
    Foo(i32),
    End
}


fn main() {
    // A thread on which background workers will be run.
    let worker_thread = thread::spawn(move || {
        let mut core = Core::new().unwrap();


        //        let z = testSimulateRemoteGetValue(&(core.handle()), |()| { State::End }).then(|_| {
        //            println!("XXXXXXXXXXXXX");
        //            futures::future::ok::<(), ()>(())
        //        });
        //        core.handle().spawn(z);

        fn doFoo(handle: Handle) -> Box<Future<Item = State, Error = ()>> {
            println!("Starting dofoo >>>> =======================================");
            let f = testSimulateRemoteGetValue(&handle, |()| { State::End });
            //Box::new(f.map(|x| { State::End }).map_err(|e| { () }))
            Box::new(f.map(|x| {
                println!("completed dofoo <<<< =======================================");
                State::End
            }).map_err(|e| { () }))
        }

        fn update(state: State) -> Cmd<State> {
            match state {
                State::Start(s) => {
                    println!("start - foo in 1 sec");
                    Cmd::After(Duration::new(1, 0), State::Foo(20))
                },
                State::Foo(s) => {
                    println!("foo - run doFoo now");
                    Cmd::Run(doFoo)
                },
                State::End => {
                    println!("end - end, run end egain in 1 sec");
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


fn testSimulateRemoteGetValue<R, F>(handle: &Handle, f: F) -> Map<Timeout, F>
    where R: Clone, F: FnOnce(()) -> R,
{
    let timeout = Timeout::new(Duration::from_millis(5000), handle).unwrap();
    let fut: Map<Timeout, F> = timeout.map(f);
    fut
}