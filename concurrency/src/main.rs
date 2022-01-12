use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::Mutex;

fn main() {
    // mpsc = multiple producer, single consumer
    // returns a tuple, 1st elem sending end 2nd element receiving end
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),          
        ];

        for val in vals {
            tx1.send(val).unwrap();
            // pause the thread 1 sec for each iteration
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),        
        ];

        for val in vals {
            tx.send(val).unwrap();
            // pause the thread 1 sec for each iteration
            thread::sleep(Duration::from_secs(1));
        }
    });

    // the receiving end got recv and try_recv methods
    // recv: wait until a value is sent down the channel then returns Result<T, E>.
    // try_recv: does not block but returns Result<T, E> immediately
    
    // rx is treated as an iterator here instead of being called directly 
    for received in rx {
        println!("Got: {}", received);
    }


    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if join is called here it's gonna wait for the thread to finish before executing the rest of the code
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // makes sure the spawned thread finishes before main exits
    handle.join().unwrap();
}
