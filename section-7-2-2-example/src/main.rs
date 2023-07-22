use std::collections::HashMap;
// use std::fmt;
// use std::io;
// use std::io::Result as IoResult;

use section_7_2_2_example::{breakfast, eat_at_restaurant};

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    eat_at_restaurant();
    let breakfast = breakfast(&"Rye");

    println!("returned breakfast {:?}", breakfast)
}

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//   // --snip--
// }
