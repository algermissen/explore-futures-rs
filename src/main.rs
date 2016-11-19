extern crate futures;

use futures::future::*;
use std::time::Duration;


// A is an example for some 'component' that
// has internal state, periodically updates that state
// and allows future-based read access to the state.
pub struct A {
    state: i32,
}

impl A {
    // interval specifies how often the internal work
    // is to be done.
    pub fn new(interval: Duration) -> A {
        A { state: 0 }
        // How to start periodic work here?
    }
    // To be called each 'interval' to update state
    fn update_state(&mut self) {
        self.state = 42;
    }
    // Provide access that involves reading the internal state.
    // TODO: make this return a future of i32
    fn get(&self) -> i32 {
        self.state
    }
}

fn main() {
    let a1 = A::new(Duration::new(5, 0));
    let a2 = A::new(Duration::new(7, 0));
    let a3 = A::new(Duration::new(25, 0));

    println!("State: a1:{} a2:{}, a3:{}", a1.get(), a2.get(), a3.get())

    //    let f1 = ok::<u32, u32>(1);
    //    let f2 = ok::<u32, u32>(2);
    //
    //
    //    let f3 = f1.and_then(|| { f2 });
}
