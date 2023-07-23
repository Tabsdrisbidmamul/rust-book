fn main() {
    let mut s = "foo".to_string();

    s.push_str("bar");

    s.push('l');

    println!("string is {} ", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("----------------------");
    println!("concatenation {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("----------------------");
    println!("format! {s}");

    let s1 = String::from("Здравствуйте");
    let h = &s1[0..2];

    println!("----------------------");
    println!("string slice {}", h);

    println!("----------------------");

    for c in "Зд".chars() {
        println!("{c}");
    }

    println!("----------------------");

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
