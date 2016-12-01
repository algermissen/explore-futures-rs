extern crate futures;
extern crate tokio_core;
extern crate wrk;

use std::time::Duration;
use std::thread;
use std::boxed::Box;

use futures::Future;
use futures::Map;
use futures::stream::Stream;

use tokio_core::reactor::Interval;
use tokio_core::reactor::Handle;
use tokio_core::reactor::Core;
use tokio_core::reactor::Timeout;

use wrk::Worker;
use wrk::Cmd;

// States of our worker's state machine
#[derive(Copy, Clone, Debug)]
enum State {
    Start(i32),
    Counting(i32),
    End
}

fn main() {
    // A thread on which background workers will be run.
    let worker_thread = thread::spawn(move || {
        // Event loop for background work on this thread
        let mut core = Core::new().unwrap();

        // Asynchronously obtains a value from somewhere and increments a counter
        fn do_count(handle: Handle, state: State) -> Box<Future<Item = State, Error = ()>> {
            match state {
                State::Counting(c) => {
                    println!("    Counting ran {:?} times", c.clone());
                    let f = simulate_get_remote(&handle, |()| { 42 });
                    Box::new(f.map(move |value| {
                        println!("    Got remote value {:?} ", value);
                        State::Counting(c + 1)
                    }).map_err(|_| { () }))
                },
                _ => panic!("Wrong state")
            }
        }

        // The internal state machine of our worker instance
        fn counter_worker_func(state: State) -> Cmd<State> {
            match state {
                State::Start(s) => {
                    println!("Starting");
                    Cmd::After(Duration::new(1, 0), State::Counting(s.clone()))
                },
                State::Counting(s) => {
                    if s < 4 {
                        Cmd::Run(State::Counting(s.clone()), do_count)
                    } else {
                        Cmd::After(Duration::new(1, 0), State::End)
                    }
                },
                State::End => {
                    println!("Ended - starting over in 10s");
                    Cmd::After(Duration::new(10, 0), State::Start(1))
                },
            }
        }

        // Create a background worker
        let _ = Worker::new(core.handle(), State::Start(1), counter_worker_func);

        // An interval stream to drive the background worker futures.
        let interval_stream = Interval::new(Duration::new(60, 0), &(core.handle())).unwrap();
        let stream_future = interval_stream.for_each(|_| {
            println!("Worker driver says hello after 60s");
            Ok(())
        });

        core.run(stream_future).unwrap();
    });

    // Request handler threads to be started here (e.g. HTTP server)
    // These threads will eventually use the state the workers work on.

    worker_thread.join().unwrap();
}


// This function simulates an async call to obtain a value, eg from
// an upstream server.
fn simulate_get_remote<R, F>(handle: &Handle, f: F) -> Map<Timeout, F>
    where R: Clone, F: FnOnce(()) -> R,
{
    let timeout = Timeout::new(Duration::from_millis(5000), handle).unwrap();
    let fut: Map<Timeout, F> = timeout.map(f);
    fut
}