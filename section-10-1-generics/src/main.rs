fn main() {
    let number_list = vec![34, 50, 25, 65, 100];
    let number_list_2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let first_largest = largest_value(&number_list);
    let second_largest = largest_value(&number_list_2);

    println!("The first_largest largest number is {}", first_largest);
    println!("The second_largest largest number is {}", second_largest);

    println!("----------------");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_value(&char_list);
    println!("The largest char is {}", result);

    println!("----------------");

    // let integer = Point { x: 5, y: -10.0 };
    // let float = Point { x: 1.0, y: -1.0 };
    // let p = Point { x: 5, y: 10 };

    // println!("integer {:?}", integer);
    // println!("float {:?}", float);
    // println!("p.x = {}", p.x());
    // println!("p.y = {}", p.y());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

#[derive(Debug)]
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// impl<T, U> Point<T, U> {
//     fn x(&self) -> &T {
//         &self.x
//     }

//     fn y(&self) -> &U {
//         &self.y
//     }
// }

// impl Point<i64, f64> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in &list[1..] {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in &list[1..] {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn largest_value<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }

    largest
}
