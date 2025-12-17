use adder_improved;
pub enum Cases {
    I64(adder_improved::Example<i64>),
    F64(adder_improved::Example<f64>)
}
pub fn setup()-> std::collections::HashMap<String, Cases>{
    let mut map: std::collections::HashMap<String, Cases> = std::collections::HashMap::new();
    map.insert("i64".to_string(), Cases::I64(adder_improved::Example{value: 2i64}));
    map.insert("f64".to_string(), Cases::F64(adder_improved::Example{value: 2.0f64}));
    map
}