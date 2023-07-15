fn main() {
    println!("Hello, world!");

    let str = String::from("Hello world");

    let str = another_function(str);

    println!("str is {str}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn another_function(x: String) -> String {
    println!("value of x {x}");
    return x;
}
