use std::any::type_name;

fn print_type<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

fn main() {
    println!("Demonstrating various signed integer types in Rust:");
    let mut int8: i8 = -42;
    println!("The value of int8 is: {int8}");
    print_type(&int8);
    let mut int16: i16 = -59;
    println!("The value of int16 is: {int16}");
    print_type(&int16);
    let mut int32: i32 = -123456;
    println!("The value of int32 is: {int32}");
    print_type(&int32);
    let mut int64: i64 = -123456789;
    println!("The value of int64 is: {int64}");
    print_type(&int64);
    let mut intarch: isize = -12345;
    println!("The value of intarch is: {intarch}");
    print_type(&intarch);


    println!("\nDemonstrating various unsigned integer types in Rust:");
    let mut uint8: u8 = 42;
    println!("The value of uint8 is: {uint8}");
    print_type(&uint8);
    let mut uint16: u16 = 59;
    println!("The value of uint16 is: {uint16}");
    print_type(&uint16);
    let mut uint32: u32 = 123456;
    println!("The value of uint32 is: {uint32}");
    print_type(&uint32);
    let mut uint64: u64 = 123456789;
    println!("The value of uint64 is: {uint64}");
    print_type(&uint64);
    let mut uintarch: usize = 12345;
    println!("The value of uintarch is: {uintarch}");
    print_type(&uintarch);

    println!("\nDemonstrating integer arithmetic operations:");
    println!("Using two i32 integers, cannot perform operations between different types without casting.");
    let a: i32 = 15;
    let b: i32 = 4;
    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    let quotient = a / b;
    let remainder = a % b;
    println!("a = {a}");
    print_type(&a);
    println!("b = {b}");
    print_type(&b);
    println!("Sum: {sum}");
    println!("Difference: {difference}");
    println!("Product: {product}");
    println!("Quotient: {quotient}");
    println!("Integer Division is used, so no fractional part.");
    println!("Remainder: {remainder}");


    println!("\nDemonstrating integer literals in different bases:");
    let mut int_hex: i32 = 0xFF;
    println!("The value of int_hex (0xFF) is: {int_hex}");
    print_type(&int_hex);
    let mut int_bin: i32 = 0b1010;
    println!("The value of int_bin (0b1010) is: {int_bin}");
    print_type(&int_bin);
    let mut int_oct: i32 = 0o77;
    println!("The value of int_oct (0o77) is: {int_oct}");
    print_type(&int_oct);
    let mut int_byte: u8 = b'A'; // byte literal, only for u8
    println!("The value of int_byte (b'A') is: {int_byte}");
    print_type(&int_byte);

    println!("\nDemonstrating floating-point types in Rust:");
    let mut float32: f32 = 3.14;
    println!("The value of float32 is: {float32}");
    print_type(&float32);
    let mut float64: f64 = 2.718281828459045;
    println!("The value of float64 is: {float64}");
    print_type(&float64);

    println!("\nDemonstrating floating-point arithmetic operations:");
    let x: f64 = 5.5;
    let y: f64 = 2.0;
    let float_sum = x + y;
    let float_difference = x - y;
    let float_product = x * y;
    let float_quotient = x / y;
    println!("x = {x}");
    print_type(&x);
    println!("y = {y}");
    print_type(&y);
    println!("Sum: {float_sum}");
    println!("Difference: {float_difference}");
    println!("Product: {float_product}");
    println!("Quotient: {float_quotient}");

    println!("\nDemonstrating boolean type in Rust:");
    let mut bool_true = true;
    println!("The value of bool_true is: {bool_true}");
    print_type(&bool_true);
    if bool_true {
        println!("bool_true is true!");
    } else {
        println!("bool_true is false!");
    }
    let mut bool_false: bool = false;
    println!("The value of bool_false is: {bool_false}");
    if bool_false {
        println!("bool_false is true!");
    } else {
        println!("bool_false is false!");
    }
    print_type(&bool_false);
    println!("\nDemonstrating character type in Rust:");
    let c = 'z';
    println!("The value of c is: {c}");
    print_type(&c);
    let z: char = 'ℤ';
    println!("The value of z is: {z}");
    print_type(&z);
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");
    print_type(&heart_eyed_cat);

    println!("\nDemonstrating compound types in Rust:");
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tuple is: {:?}", tuple);
    print_type(&tuple);
    println!("Accessing tuple elements by index:");
    println!("The first element (tuple.0) is: {}", tuple.0);
    println!("The second element (tuple.1) is: {}", tuple.1);
    println!("The third element (tuple.2) is: {}", tuple.2);
    let (x, y, z) = tuple;
    println!("Assigning (x,y,z) = tuple");
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of array is: {:?}", array);
    print_type(&array);
    println!("Accessing array elements by index:");
    for i in 0..array.len() {
        println!("Element at index {i} is: {}", array[i]);
    }
    println!("Initializing an array with the same value:");
    let array_same: [i32; 5] = [3; 5];
    println!("The value of array_same (array_same = [3; 5]) is: {:?}", array_same);
    print_type(&array_same);

    println!("\nArray of various types cannot be created directly in Rust, must use enums or tuples for mixed types:");
    let mixed_tuple: (i32, f64, char) = (42, 3.14, 'R');
    println!("The value of mixed_tuple is: {:?}", mixed_tuple);
    print_type(&mixed_tuple);
    // Using enum to create an array of mixed types
    #[derive(Debug)]
    enum MixedType {
        Int(i32),
        Float(f64),
        Char(char),
    }
    let mixed_array: [MixedType; 3] = [
        MixedType::Int(42),
        MixedType::Float(3.14),
        MixedType::Char('R'),
    ];
    println!("The value of mixed_array is: {:?}", mixed_array);
    print_type(&mixed_array);

    println!("\nOr use a Vec to hold mixed types via enums:");
    let mut mixed_vec: Vec<MixedType> = Vec::new();
    mixed_vec.push(MixedType::Int(100));
    mixed_vec.push(MixedType::Float(6.28));
    mixed_vec.push(MixedType::Char('Z'));
    println!("The value of mixed_vec is: {:?}", mixed_vec);
    print_type(&mixed_vec);

    println!("\nArray of strings:");
    let string_array = ["Hello", "from", "Rust"];
    println!("The value of string_array is: {:?}", string_array);
    print_type(&string_array);

    println!("\nEnd of type demonstrations.");

}
