fn main() {
    use std::collections::HashMap;
    // This declaration is okay because we use insert. The compiler will know what types to use.
    // It is better to be explicit:
    // let mut scores: Hashmap<String, i32> = Hashmap::new();
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
    // To iterate over a Hashmap we iterate over its reference:

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hashmaps also take ownership of variables that do not have the Copy trait:

    let s = String::from("Red");

    scores.insert(s,100);
    println!("{:?}",scores);
    // s is no longer usable

    // Hashmap values can be overwritten
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // And we can update a key's value only if it isn't present
    scores.entry(String::from("Blue")).or_insert(5000);
     //Entry method

    // We can also modify the entries directly. or_insert() returns a reference
    // to the value itself.
    let count = scores.entry(String::from("Blue")).or_insert(50);
    *count+=1; // 26
    println!("{count}");

    //But we can also pull the reference with .get_mut()
    let count = scores.get_mut(&String::from("Blue")); //Option<&i32>
    match count {
        Some(i) => *i+=1, // 27
        None => ()
    }

    // Whereas .get() returns an unmutable reference

    let count = scores.get(&String::from("Blue"));
    match count {
        Some(i) => println!("{i}"),//27
        None => ()
    }

    //Here is an example of using a hashmap to count the occurrences of words in a string literal
    // This also showcases the iterator that the .split_whitespace() method returns

    let text = " Hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }
    println!("{:?}", map);

}
