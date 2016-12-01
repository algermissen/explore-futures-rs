extern crate futures;
extern crate tokio_core;

use std::time::Duration;
use std::result::Result;
use std::boxed::Box;

use futures::Future;

use tokio_core::reactor::Handle;
use tokio_core::reactor::Timeout;

#[derive(Debug)]
pub struct Error {
    code: i32
}

impl Error {
    pub fn new(c: i32) -> Error {
        Error { code: c }
    }
}

// The instructions that the 'Worker' understands
// Commands return a new state to drive the next action
pub enum Cmd<S: Clone> {
    // Start with a certain state
    Start(S),
    // Run a function that returns a future of the state to proceed with
    //    Run(fn(handle: Handle) -> Box<Future<Item = S, Error = ()>>),
    Run(S, fn(handle: Handle, S) -> Box<Future<Item = S, Error = ()>>),
    // Switch to a new state after duration
    After(Duration, S)
}


// A struct that holds some state and a handle on
// which to run its worker function.
#[derive(Debug)]
pub struct Worker<S: Clone> {
    //dur: Duration,
    state: S,
    //fu: fn(S) -> Cmd<S>,
}

// An actor takes an internal state machine (guts) which is a function
// from state to command-resulting-in-state
impl<S: 'static + Clone> Worker<S> {
    pub fn new(handle: Handle, start_state: S, guts: fn(S) -> Cmd<S>) -> Worker<S> {
        //        let d = Duration::from_millis(5000);
        //        let a: Worker<S> = Worker { dur: d, state: start.clone(), fu: f };
        let a: Worker<S> = Worker { state: start_state.clone() };
        let cmd = Cmd::Start(start_state.clone());
        Worker::run_guts(handle, cmd, guts);
        a
    }

    // execute the provided internals (guts)
    fn run_guts(handle: Handle, cmd: Cmd<S>, guts: fn(S) -> Cmd<S>) {
        match cmd {
            Cmd::Start(state) => {
                let timeout = Timeout::new(Duration::from_millis(5000), &handle).unwrap();
                let handle_copy = handle.clone();
                let state_copy = state.clone();
                let guts_copy = guts.clone();
                let fut = timeout.then(move |_| {
                    //                    println!("******************************** START **********");
                    let next_cmd = guts_copy(state_copy);
                    Worker::run_guts(handle_copy, next_cmd, guts_copy);
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
                let fut = timeout.then(move |_| {
                    //                    println!("******************************** AFTER **********");
                    let next_cmd = guts_copy(state_copy);
                    Worker::run_guts(handle_copy, next_cmd, guts_copy);
                    // TODO match on r
                    futures::future::ok::<(), ()>(())
                });
                handle.spawn(fut);
            },
            //            Cmd::Run(work_future) => {
            //                let handle_copy = handle.clone();
            //                let handle_copy2 = handle.clone();
            //                //                let state_copy = state.clone();
            //                let guts_copy = guts.clone();
            //                let fut0: Box<Future<Item = S, Error = ()>> = work_future(handle_copy);
            //                let fut = fut0.then(move |x: Result<S, ()>| {
            //                    let new_state = x.unwrap();
            //                    //                    println!("******************************** RUN **********");
            //                    let next_cmd = guts_copy(new_state);
            //                    Worker::run_guts(handle_copy2, next_cmd, guts_copy);
            //                    // TODO match on r
            //                    futures::future::ok::<(), ()>(())
            //                });
            //                handle.spawn(fut);
            //            },
            Cmd::Run(state, work_future) => {
                let handle_copy = handle.clone();
                let handle_copy2 = handle.clone();
                let state_copy = state.clone();
                let guts_copy = guts.clone();
                let fut0: Box<Future<Item = S, Error = ()>> = work_future(handle_copy, state_copy);
                let fut = fut0.then(move |x: Result<S, ()>| {
                    let new_state = x.unwrap();
                    //                    println!("******************************** RUN **********");
                    let next_cmd = guts_copy(new_state);
                    Worker::run_guts(handle_copy2, next_cmd, guts_copy);
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

