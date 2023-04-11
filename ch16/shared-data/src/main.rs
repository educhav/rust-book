use std::sync::{Arc,Mutex};
use std::rc::Rc;
use std::thread;

fn main() {
    let m = Mutex::new(5);

    {
        // The call to lock would fail if another thread holding the lock panicked. 
        // In that case, no one would ever be able to get the lock, so we’ve chosen 
        // to unwrap and have this thread panic if we’re in that situation.
        //
        //
        // Rust's type system makes it so that we must call lock() before mutating the value since
        // the type if Mutex<i32> and not i32
        //
        // lock() returns a LockResult with a MutexGuard inside
        // MutexGuard implements Deref to let us point to the inner value
        // MutexGuard implements Drop that releases the lock when it goes out of scope
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    // Lock release happens
    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Use Atomic refcount to safely send Rcs between threads
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
