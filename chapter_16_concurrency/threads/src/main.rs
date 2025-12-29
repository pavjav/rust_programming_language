///
/// Concurrency is about different parts of a program executing independently
/// Parallelism is about different parts of a program executing simultaneously

use std::thread;
use std::time::Duration;

fn main() {
    println!("Running 1 spawned thread alongside the main thread");
    println!("The main thread will cut off the spawned thread before it can finish looping");
    thread::spawn( || {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }
    );

    for i in 1..5 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    println!("Running 1 spawned thread alongside the main thread with a handle to join afterwards.");
    println!("The main thread will cut off the spawned thread before it can finish looping");
    let handle = thread::spawn( || {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }
    );

    for i in 1..5 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    println!("Running 1 spawned thread alongside the main thread with a handle to join between the spawned thread and the main one.");
    println!("The main thread will cut off the spawned thread before it can finish looping");
    let handle = thread::spawn( || {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }
    );

    handle.join().unwrap();

    for i in 1..5 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    thread::sleep(Duration::from_millis(50));

    // How to use the move keyword in a thread closure to give ownership to the closure

    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("Here is vector v in thread: {:?}", v);
    });
    handle.join().unwrap();

    // We cannot use v once it has been passed to an inner thread
    // It can be unsafe for multiple threads to own a variable
    // To pass data between inner and outer threads, we can use channels
    use std::sync::mpsc;

    let (tx,rx) = mpsc::channel();

    let v  = vec![1,2,3];
    let handle = thread::spawn(move ||
        {
            println!("Here is vector v in thread: {:?}", v);
            tx.send(v).unwrap();
        }
    );
    let received_v = rx.recv().unwrap();
    println!("Here is vector v received from thread: {:?}", received_v);

    //recv() implicitly pauses the main thread and waits for something to be sent before 
    //We can also use rx.try_recv() which will not wait

    // We can also iterate over rx to receive messages as they become available, e.g.
    let (tx,rx) = mpsc::channel();
    thread::spawn(move ||
    {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }
    );

    for received in rx {
        println!("Got: {received}");
    }

    // To have multiple transmitters, we can clone the original transmitter:

    let (tx,rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move ||
    {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }
    );

    thread::spawn(move ||
    {
        let vals = vec![
            String::from("Here"),
            String::from("are"),
            String::from("more"),
            String::from("messages")
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }
    );

    for received in rx {
        println!("Got: {received}");
    }
    

    
}
