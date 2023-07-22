#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut number_vector = Vec::<i8>::with_capacity(1);

    number_vector.push(10);

    println!("number_list {:?}", number_vector);
    println!("number_list capacity {}", number_vector.capacity());
    println!("number_list length {}", number_vector.len());

    println!("---------------------");

    number_vector.push(20);

    println!("number_list {:?}", number_vector);
    println!("number_list capacity {}", number_vector.capacity());
    println!("number_list length {}", number_vector.len());

    println!("---------------------");

    let mut initialised_vector = vec![1, 2, 3];

    println!("initialised_vector {:?}", initialised_vector);

    println!("---------------------");

    let value_as_index = &initialised_vector[1];

    println!("value as index: {}", value_as_index);

    println!("---------------------");

    let value_as_option = initialised_vector.get(1);

    match value_as_option {
        Some(value) => println!("value as option {} ", value),
        None => println!("there is no value"),
    };

    println!("---------------------");

    for i in &initialised_vector {
        println!("value at loop {i}");
    }

    println!("---------------------");

    for i in &mut initialised_vector {
        *i *= 10;
        println!("value at loop after mutation {i}");
    }

    println!("---------------------");

    println!("initialised_vector after mutation {:?}", initialised_vector);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("---------------------");

    println!("row elements {:?}", row);

    initialised_vector.remove(2);

    println!("---------------------");

    println!("initialised_vector after removal {:?}", initialised_vector);
}
