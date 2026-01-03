
//Using C ABI
unsafe extern "C" {
    unsafe fn abs(input: i32) -> i32;
}


// Static variables exist in a fixed location in memory, and can be made mutable or unmutable
// To use a static variable is safe. To modify a mutable one is unsafe
// static variables exist in the `static lifetime
// static variable
static HELLO_WORLD: &str = "Hello, world";

//static mutable variable
static mut COUNTER: u32 = 0;

//unsafe function
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

//safe function
fn print_hello () {
    println!("{HELLO_WORLD}");
}

// unsafe and as of 2024 need to allow static_mut_refs
#[allow(static_mut_refs)]
fn print_count () {
    unsafe {
        println!("{}", COUNTER);
    }
}

// we can also label traits as unsafe
unsafe trait Foo{
    fn bar(&self);
}

// and create unsafe implementations for them
unsafe impl Foo for i32 {
    fn bar(&self) {
        let r1 = self as *const i32;
        unsafe {
            println!("Integer is: {}", *r1);
        }
    }
}

fn main() {
    // A raw pointer is declared as *const or a *mut type for a constant/mutable reference to it
    // Unlike smart pointers, we can make multiple raw pointers
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Dereferencing however must be called within an unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Array types have a safe pointer method for splitting an array
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a,b) = r.split_at_mut(3);
    assert_eq!(a,&mut [1,2,3]);
    assert_eq!(b,&mut [4,5,6]);

    // Now a, b are &mut [i32] types that point to subarrays of v split at a specific index (3).
    // Basically this splits an array into [0:3],[3:]
    // T0 do the same with a raw pointer, we can create a safe abstraction over unsafe code
    // See fn split_at_mut below

    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a,b) = split_at_mut(r, 3);
    assert_eq!(a,&mut [1,2,3]);
    assert_eq!(b,&mut [4,5,6]);

    // The extern import can be used to directly use the C ABI at assembly level
    // It is inherently unsafe, but we can pass values to it as below: 

    unsafe {
        println!(
            "Absolute value of -3 according to C: {}",
            abs(-3)
        )
    }

    // Here we use some safe/unsafe functions with static variables
    // safe
    print_hello();

    //unsafe
    print_count();
    add_to_count(3);
    print_count();

    // Unsafe implementation for i32
    let x = 1;
    x.bar();


}

use std::slice;

fn split_at_mut(
    values: &mut [i32],
    mid: usize,
)-> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr(); // *mut i32 raw pointer
    assert!(mid <= len);
    // wrap in unsafe block
    unsafe {
        (
            // this function will return a mutable smart pointer to a sub array of size mid
            slice::from_raw_parts_mut(ptr, mid),
            // the .add(count) method is used to shift the beginning address by data_size*count
            // ptr.add(count) returns another raw pointer
            slice::from_raw_parts_mut(ptr.add(mid), len-mid)
        )
    }

}