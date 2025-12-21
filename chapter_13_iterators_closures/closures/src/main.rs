#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(
        &self,
        user_preference: Option<ShirtColor>,
    ) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue +=1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        }
        else {
            ShirtColor::Blue
        }
    }
}


fn main() {
    println!("Hello, world!");

    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
        ]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1,
        giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2,
        giveaway2
    );

    // When we define a closure it instantiates as a Fn(Input) -> Output type
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    //When the closure types are inferred we cannot overload it ith different types
    //This will not compile
    //let n = example_closure(5);
    
    // We can use closures to mutate data by borrowing a mutable variable directly
    // This creates a FnMut() type
    let mut list = vec![1,2,3];
    let mut borrows_mutably = || list.push(4);
    borrows_mutably();

    use std::thread;

    // If we need a function to move the variable into the scope of the closure
    // we use the move keyword.
    // This is most useful when moving data so it is owned by a new thread
    let list = vec![1,2,3];
    thread::spawn(
        move || {
            println!("From thread: {:?}", list)
        }
    ).join().unwrap();

    //The unwrap_or_else method for Option<T> looks like this:
    // pub fn unwrap_or_else<F>(self,f:F)->T 
    // where
    //     F: FnOnce() -> T
    // {
    //     match self {
    //         Some(x) => x,
    //         None => f(),
    //     }
    // }
    // All closures satisfy FnOnce at least. It can move captured values/refs in and out of its body, and mutate
    // captured refs. This is typically only called once because it could take ownership and not return it.
    // 
    // By contrast FnMut applies to closures that do not move captured values outside of its body but might mutate
    // a captured ref.
    //
    // The inclusion pattern is:
    // FnMut < FnOnce < Fn
    //
    // The array method .sort_by_key() looks like
    //impl<T> [T]
    //pub fn sort_by_key<K, F>(&mut self, mut f: F)
    //where
    //    F: FnMut(&T) -> K,
    //    K: Ord,
    // { ... }
    // Here it takes an FnMut(&T) -> K because it is called multiple times 
    // In the below example we define rectangles with u32 widths
    // This makes T = Rectangle and K = u32.
    // Our function f is |r| r.width
    // Which takes a rectangle by reference and returns its u32 width.
    // Since K implements Ord, we are within the trait bounds


    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 }
    ];

    //width_closure is Fn(&Rectangle) -> u32

    let width_closure = |r: &Rectangle| r.width;
    list.sort_by_key(width_closure);
    //or
    list.sort_by_key(|r| r.width);

    // Let's try to count the number of times something is called in sort with an FnMut closure

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 }
    ];
    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("{:?} sorted in {num_sort_operations} operations", list);

    
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}


