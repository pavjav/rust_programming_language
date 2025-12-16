fn main() {
    //Static sized vectors
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1,2,3];

    //Mutable vectors
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    
    //Access by reference
    v = vec![1,2,3,4,5];
    println!("{:?}",v);
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    
    //Using .get() method to return Option types
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element.")
    }

    let fiftieth: Option<&i32> = v.get(50);
    match fiftieth {
        Some(fiftieth) => println!("The fiftieth element is {fiftieth}"),
        None => println!("There is no fiftieth element.")
    }

    //Iterate over vector by ref
    for i in &v {
        println!("{i}");
    }

    //Alter a mutable vector
    let mut v = [1,2,3];
    for i in &mut v {
        *i += 50;
    }

    //Using enums to store multiple data types in a vector

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let _row: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("hello"))
    ];



}

