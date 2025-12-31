/// Pattern matching is important for handling Enum variants like Option.
/// Closures may cut it for unexpected values, but patterns give us more flexibility with handling multiple types as well


fn main() {
    // if let expressions are handy when we need to deconstruct an Option, or evaluate a result.
    // We can use these in conjunction with other conditionals as well
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8,_> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Favorite color: {}", color);
    }
    else if is_tuesday {
        println!("It is tuesday!");
    }
    else if let Ok(age) = age {
        if age > 30 {
            println!("You're older than 30!");
        }
        else {
            println!("You're 30 or younger!");
        }
    }
    else {
        println!("Too bad no color for you.");
    }


    // while let loops are handy for doing something until completion. For example, popping a stack

    let mut stack = vec![1,2,3];
    // This will loop until we've popped every value off of the vector
    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // enumerating an iterator. an iterator can be enumerated to keep track of indexes

    let v = vec!['a','b','c'];
    for (index, value) in v.iter().enumerate() {
        println!("v[{index}] = {value}");
    }

    // Matching can be performed on literals with a default behavior

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything else")
    }

    // We can even match on variables in other scopes
    // The following will produce "Matched, y = 10"
    // This is because Some(y) is always a valid match

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x ={:?}", x)
    }

    // Multiple patterns and ranges can be achieved as well
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        _ => println!("anything else")
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("anything else")
    }

    // To destructure a struct, we can use matches to anticipate specific values
    // We can even access values in fields this way

    let p = Point {x: 0, y: 7};
    let Point{x,y} = p;
    assert_eq!(0,x);
    assert_eq!(7,y);

    match p {
        Point {x, y: 0} => println!("On x axis at {x}"),
        Point{x: 0, y} => println!("On y axis at {y}"),
        Point{x,y} => println!("On neither axis at ({x},{y})")
    }

    // Enum matching is the most common type

    let msg = Message::ChangeColor(0,0,0);

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move{x,y} => println!("Move to ({x},{y})"),
        Message::Write(s) => println!("Write: {s}"),
        Message::ChangeColor(x,y,z) => println!("Change color to RGB ({x},{y},{z})")
    }

    // In case we nest a struct into an enum variant, say if ChangeColor(RGB) with a struct called RGB
    // Then we would write:
    // Message::ChangeColor(RGB(r,g,b)) => ...

    // Match guards ensure we satisfy additional conditions and allows us to match on values on nested types

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Defualt case, x = {:?}",x)
    }

}

enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

struct Point {
    x: i32,
    y: i32
}