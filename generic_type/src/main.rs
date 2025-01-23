/// @Author: Mitul Tyagi
/// @Date:   2024-11-28 22:26:10
/// @Description: Generic Type and Traits
/*
- Generics are used to create definitions for items like function signatures or structs, which can then be used with many different concrete data types.
- When defining a function that uses generics, the generics is placed in the signature of the function where the data types of the parameters and return value are
  specified. Doing so makes the code more flexible and provides more functionality to callers of the function while preventing code duplication.
- Structs and Enums can be defined to use a generic type parameter in one or more fields using the <> syntax.
- Methods can be implemented on structs and enums with generic types in their definitions too.
- Rust performs monomorphization of the code using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in
  the concrete types that are used when compiled.
*/
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point_New<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
struct PointCart<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointCart<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let both_integer = Point_New { x: 5, y: 10 };
    let both_float = Point_New { x: 1.0, y: 4.0 };
    let integer_and_float = Point_New { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p1 = PointCart { x: 5, y: 10.4 };
    let p2 = PointCart { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
