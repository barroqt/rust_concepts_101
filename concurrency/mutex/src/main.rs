use std::thread;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

fn main() {
    // Mutex = mutual exclusion | ARC = Atomic Reference Counted (type)
    // 2 rules:
    // 1. You must attempt to acquire the lock before using the data.
    // 2. When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
    // Smart pointer that allows threads to talk one after the other

    // Single thread
    let m = Mutex::new(5);

    {
        // lock returns a smart pointer called MutexGuard, wrapped in a LockResult
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // Multiple threads

    // will call lock and add one to the mutex after each thread
    // We need to wrap the Mutex<T> in Rc<T> , but Rc<T> is not safe for concurrent situations so we wrap it in Arc<T>...
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // launch 10 threads
    for _ in 0..10 {
        // ... and clone the Arc<T> before moving ownership to the thread.
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