// We use a Box<T> to point to data on the heap.
// This is helpful in the following contexts:
// 1. When you have a type whose size is unknown at compile time, but you want to use a value of that type that
// requires an exact size
// 2. When you have a large amount of data and need to transfer ownership
// 3. When you want to own a value and you only care that it implements a specific trait


// It also helps in recursive definitions. Here Cons takes either an int or points to another List.
// This is a linked list:
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// To create our own pointer we must alsp implement std::ops::Deref in order to access its value, e.g.

struct MyBox<T>(
    T
);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Now *z will actually call *(z.deref()) since z.deref() returns an &T type

// Deref Coercion happens when one type can be interpreted as another. Look at the following function

fn hello(name: &str) {
    println!("Hello, {name}!");
}


// This will add one to an int

fn add_one(x: &mut i32) {
    *x+=1;
}

// Here we implement DerefMut for MyBox

impl<T> std::ops::DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

enum RcList {
    Cons(i32, std::rc::Rc<RcList>),
    Nil,
}


#[derive(Debug)]
enum MutList {
    Cons(std::rc::Rc<std::cell::RefCell<i32>>, std::rc::Rc<MutList>),
    Nil,
}


fn main() {
    // Box for recursive types
    use List::Cons;
    use List::Nil;
    let lst = Cons(1, Box::new(Cons(2, Box::new(Nil))));

    // A regular reference can be dereferenced with the * operator
    let x = 5;
    let y = &x;

    assert_eq!(x,5);
    assert_eq!(*y, 5);

    // A box can be dereferenced as well.
    // However, Box::new(x) is a pointer to a copy of x, not x itself
    
    let z = Box::new(x);
    assert_eq!(5, *z);

    // For our Box implementation,
    let z = MyBox::new(x);
    assert_eq!(5,*z);

    // This pointer of type String can be passed via reference to a function fn hello(name: &str) -> ()
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // The reason this works is because we implemented deref. Since the contents of MyBox gets derefed as a &String
    // We can pass the box by reference and it will automatically coerce the type as necessary
    // Likewise, &String is coerced as &str.
    // This is because a String holds a dynamically allocated str and implements deref so that dereferencing a String will
    // Point to the contents of the underlying string literal

    // There are three cases where deref coercion occurs
    // 1. From &T to &U when T: Deref<Target=U>
    //      Like with MyBox and String
    // 2. From &mut T to &mut U when T: DerefMut<Target=U>
    // 3. From &mut T to &U when T: Deref<Target=U>
    // 1,2 are pretty self explanatory but 3 is tricky
    // We can coerce from a mutable to an immutable but not the other way around

    // Here we see coercion of &mut MyBox<i32> to underlying &mut i32 

    let mut a = MyBox::new(5);
    add_one(&mut a);

    // Boxes are useful when ownership is clear. When multiple ownerships of a pointer can occur, we use the Rc<T>.
    // The Reference Counted Smart Pointer 
    // Below we look at a connecting two graphs to an existing one by reference.
    // a: 5 -> 10 -> Nil
    // b: 3 -> a
    // c: 4 -> a
    let a = std::rc::Rc::new(
        RcList::Cons(5, std::rc::Rc::new(
            RcList::Cons(10,std::rc::Rc::new(
                RcList::Nil)
            )
        )
    )
    );

    println!(
        "Count after creating a = {}",
        std::rc::Rc::strong_count(&a)
    );

    let b = RcList::Cons(3, std::rc::Rc::clone(&a));

    println!(
        "Count after creating b = {}",
        std::rc::Rc::strong_count(&a)
    );

    let c = RcList::Cons(4, std::rc::Rc::clone(&a));

    println!(
        "Count after creating c = {}",
        std::rc::Rc::strong_count(&a)
    );

    // std::rc::Rc::clone(&a) is of type Rc<RcList> which is a valid type for Cons.
    // Rc only allows immutable borrows, whereas Box allows both mutable and immutable borrows
    // The last pointer type is the RefCell, and allows for both types of borrows, but has a feature called interior
    // mutability.
    // This is considered unsafe because we can trigger a runtime panic and force mutability on otherwise immutable variables
    // Typically this is done when private attributes need to be updated.
    // Another consideration is that RefCell is only to be used in single-thread
    // Check lib.rs for a RefCell use case

    // In summary:
    // Box<T> : Single owner, multi-threadable, mutable/immutable borrows
    // Rc<T> : Multiple owners, single thread, only immutable borrows
    // RefCell<T> : Single ownner, single thread, mutable/immutable borrows, interior mutability

    // We can allow multiple owners of mutable data by combining Rc and RefCell. For example:
    // value: 5
    // a = value -> Nil
    // b = 3 -> a
    // c = 4 -> a

    let value = std::rc::Rc::new(std::cell::RefCell::new(5));

    let a = std::rc::Rc::new(MutList::Cons(std::rc::Rc::clone(&value), std::rc::Rc::new(MutList::Nil)));
    let b = MutList::Cons(std::rc::Rc::new(std::cell::RefCell::new(3)), std::rc::Rc::clone(&a));
    let c = MutList::Cons(std::rc::Rc::new(std::cell::RefCell::new(4)), std::rc::Rc::clone(&a));

    *value.borrow_mut() += 10;

    //Even though there is only one mutable borrow of value, a, b, and c "own" it.

    println!("a:{:?}", a);
    println!("b:{:?}", b);
    println!("c:{:?}", c);


    
}