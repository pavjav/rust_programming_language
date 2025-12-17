/*
Lifetimes for referenes must be made explicit:
&i32 -> reference
&'a i32 -> reference with explicit lifetime
&'a mut i32 -> reference to mutable with explicit lifetime
*/

// Here we require both x and y to have the same lifetime
// This can cause issues.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

// Look at this example with different lifetimes
// 'a, 'b, are all different lifetimes
// We can set it so they all require eachother in the where clause
// We then return it to with the lifetime of the left hand side
// We have no choice but to have both lifetimes require each other
// Because we are returning a reference to one.
// We cannot get around this unless our only intention is to mutate one reference or another
fn better_longest<'a, 'b >(x: &'a str, y: &'b str) -> &'a str
where
    'a: 'b,
    'b: 'a
{
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

// We can use lifetimes in structs as well
struct Example <'a,'b> {
    part_a: &'a str,
    part_b: &'b str
}

//Putting it all together:

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where
    T: std::fmt::Display
{
    println!("Announcement: {ann}");
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

fn main() {
    let s1 = String::from("long, long line");
    //let result;
    {
        let s2 = String::from("short string");
        // This will fail because s2 and s1 have different lifetimes
        // result, s1 have a longer lifetime than s2
        // But result cannot be coerced into extending its lifetime 
        //result = longest(s1.as_str(), s2.as_str());
    }
    //println!("{result}");

    let s1 = String::from("long, long line");
    let result;
    {
        let s2 = String::from("short string");
        let s2_str = s2.as_str();
        result = better_longest(s1.as_str(), s2_str);
        println!("{result}");
    }
    //println!("{result}")
    // This fails because 'a requires that 'b outlives it in this function

    let s1 = String::from("foo");
    let ex = {
        let s2 = String::from("bar");
        let ex = Example{part_a: s1.as_str(),part_b: s2.as_str()};
        //ex
        // If we tried to return ex to the ex in the outer scope, we get an error that s2 does not live long enough
        // Even though we specified two lifetimes in the struct, we cannot coerce it to 
        // keep a reference to an object that is dropped outside of this block.
        // That would result in a dangling reference, which creates safety concerns.
        // Rust specifically d
    };

    // Static lifetimes live for the duration of the program. It is a special lifetime meant to persist and contain all others
    
    let s: &'static str = "I have a static lifetime";

    // Below we are using our generic function with lifetimes

    let s1 = String::from("long, long line");
    let s2 = String::from("bar");
    let result = longest_with_announcement(s1.as_str(),s2.as_str(), "Yay!");


}
