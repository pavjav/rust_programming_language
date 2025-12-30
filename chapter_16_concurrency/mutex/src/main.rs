/// Mutexes or Mutual Exclusions are for using data across multiple threads
/// By way of a lock/unlock system
/// We lock the mutex until we are done working on it. Once complete, we can unlock it
/// and allow other threads to access it

use std::sync::Mutex;
use std::thread;
use std::sync::Arc;
fn main() {
    let m = Mutex::new(5);
    // Use the mutex in an inner scope.
    // The mutex does not need to be declared as mut, we can just obtain it as a mutable
    // MutexGuard type
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);


    // In order to use a mutex in numerous threads, we must wrap it in a pointer like Rc
    // Rc allows for multiple owners, and each thread needs ownership of the mutex
    // However Rc is unsafe to use concurrently and so we use Arc instead
    // Atomically reference counted pointer

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_ptr = Arc::clone(&counter);
        let handle = thread::spawn(
            move || {
                let mut num = counter_ptr.lock().unwrap();
                *num += 1;
            }
        );
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Rc does not work because Send is not implemented for it
    // Note that Mutex<T> provides interior mutability
    // Similarly, RefCell<T> allows interior mutability with mut_borrow()
    // We also wrap a Mutex<T> in an Arc<T> to allow for multithreading
    // Similarly, we wrap a RefCell<T> in an Rc<T> to allow for multiple owners
    // In summary it is no coincidence that:
    //
    // Rc<RefCell<T>> --> Allows for interior mutability of T with multiple owners
    // Arc<Mutex<T>> --> Allows for interior mutability of T with multiple threads
    

}
