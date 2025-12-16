fn main() {
    // strings can refer to &str slices or String types.
    // Strings are arrays of UTF-8 characters. Strings cannot be copied, so you have to be careful with them.

    let data = "initial contents"; //&str literal
    let s = data.to_string(); //s is a String
    let s = String::from("{data}");
    let s: String = String::from("initial contents");

    // You can push string literals to String types
    let mut s = String::from("foo");
    s.push_str("bar");

    //We can push chars 'x' as well
    let mut s = String::from("lo");
    s.push('l');

    //String concatenation is also tricky in terms of ownership
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // This does not work because + expects &str on the right hand side
    //let s3 = s1+s2;

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // This moves s1 to s3 and so s1 is no longer usable
    let s3 = s1+&s2;
    println!("{}",s2);

    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // This concatenation does not work because &String+&String is not defined
    // let s3 = &s1+&s2;

    //Typically do a += concat on a mutable String
    s1+=&s2;
    println!("{}",s1);

    //It is wiser to just use the format! macro on String types to avoid ownership issues.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}","{s1}-{s2}-{s3}"); // s is a String type
    println!("{}",s);
    // Strings are similar to vectors but they are not indexable. String literals can be sliced though:

    let s = String::from("hello");
    let s = s.to_string();
    println!("{}",&s[0..2]);

    // Because UTF-8 character sizes range from 1-4 bytes, we cannot guarantee what the substring will look like.
    // Above it prints "he" because "hello" only requires 1 byte per character.
    // Lets look at the russian word "привет"

    println!("{}",&"привет"[0..2]);

    // This prints "п" because each char requires 2 bytes.

    //To iterate over a string literal characters it is better to use the .char() method:

    for c in "привет".chars() {
        println!("{c}");
    }
    // We can even iterate over the bytes:

    for b in "привет".bytes() { //b: u8
        println!("{b}");
    }


}
