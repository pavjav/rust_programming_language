/// Closures are essentially objects that implement Fn, FnMut, and FnOnce traits
/// But functions are pointers of type fn that implement all three closure traits.
/// This means a function of functions should be generically typed as dyn Fn so it can take either
/// a closure or a function pointer

fn add_one(x: i32) -> i32 {
    x+1
}

fn do_twice( f: fn(i32)->i32, arg:i32) -> i32 {
    f(arg) + f(arg)
}

fn do_twice_generic<F, T>(f: F, arg: T) -> i32 
where 
    F: Fn(T) -> i32,
    T: Copy
{
    f(arg) + f(arg)
}

/// An Enum variant can also be used as a function pointer

enum Status {
    Value(u32),
    Stop
}

/// Lastly to return a closure we must return a pointer to an object that implements Fn

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let x = 1;
    println!("add_one(x) = {}", add_one(x));
    println!("do_twice(x) = {}", do_twice(add_one, x));
    println!("do_twice_generic(x) = {}", do_twice_generic(add_one, x));

    // The map method of an iterator is a function that can take either a closure or a function pointer
    let v = vec![1,2,3];
    // Pass a function
    let mut v_str: Vec<String> = v
        .iter()
        .map(ToString::to_string)
        .collect();
    // Or pass a closure of type FnMut(&i32) -> String
    v_str = v
        .iter()
        .map(|i| i.to_string())
        .collect();

    // Using an enum variant as an argument to map() is also allowed

    let v: Vec<Status> = (0u32..20)
        .map(Status::Value)
        .collect();

}

