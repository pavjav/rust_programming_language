use adder_improved;
mod common;
use common::setup;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder_improved::add(2,2));
    //assert_eq!(4, adder_improved::add_private(2,2)); //cannot access this private function
}

#[test]
fn instantiate_multiple_types() {
    let map = setup();
    map.get(&String::from("i64"))
        .unwrap_or_else(|| panic!("Could not instantiate i64 type"));
    map.get(&String::from("f64"))
        .unwrap_or_else(|| panic!("Could not instantiate f64 type"));
}

#[test]
#[should_panic]
fn will_fail() {
    let map = setup();
    map.get(&String::from("f32"))
        .unwrap_or_else(|| panic!("Could not instantiate f32 type"));
}