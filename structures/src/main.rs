/// @Author: Mitul
/// @Date:   2024-11-15 20:05:04
/// @Description: Structure
/*
- Structs and enums are the building blocks for creating new types in Rust.
- A struct, or structure, is a custom data type that lets user package together and name multiple related values that make up a meaningful group.
- Structs are similar to tuples, in that both hold multiple related values. Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a
  struct user names each piece of data so it’s clear what the values mean. Adding these names means that structs are more flexible than tuples: user doen’t has to
  rely on the order of the data to specify or access the values of an instance.
- To define a struct, the keyword struct is used along with the name the entire struct. A struct’s name should describe the significance of the pieces of data being
  grouped together. Then, inside curly brackets the names and types of the pieces of data, which are called fields, are defined.
- To use a struct create an instance of that struct by specifying concrete values for each of the fields. An instance can be created by stating the name of the struct
  and then add curly brackets containing key: value pairs, where the keys are the names of the fields and the values are the data to be stored in those fields.
  The fields need not be specified in the same order in which thety are decalared in the struct. In other words, the struct definition is like a general template for
  the type, and instances fill in that template with particular data to create values of the type.
- To get a specific value from a struct, dot notation is used.  If the instance is mutable, The value can be changed by using the dot notation and assigning into a
  particular field. The entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
- As with any expression, a new instance of the struct can be constructed as the last expression in the function body to implicitly return that new instance.
- The field init shorthand syntax can be used if the function parameter names are same as the structure field names.
- It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some. This can be done using the  struct
  update syntax. The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
- Rust also supports structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names
  associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when the whole tuple is given a name and make the tuple a
  different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant. To define a tuple struct, start with the struct
  keyword and the struct name followed by the types in the tuple.
- Each struct defined is its own type, even though the fields within the struct might have the same types.
- Structures can be defined with no fields. These are called unit-like structs because they behave similar to (), the unit type. Unit-like structs can be useful when
  a trait is needed to implementd on some type but don’t have any data that you want to store in the type itself.
- Methods are similar to functions: they are declared with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s
  run when the method is called from somewhere else. Unlike functions, methods are defined within the context of a struct or an enum or a trait object. Their first
  parameter is always self, which represents the instance of the struct the method is being called on. Other parameters can also be passed to the methods.
- Each struct is allowed to have multiple impl blocks.
- All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. An associated function can
  be defined that doesn’t have self as its first parameter (and thus is not a methodss) because it dosn’t need an instance of the type to work with.
- Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. These are often called new, but new isn’t a
  special name and isn’t built into the language. To call this associated function, the :: syntax is used with the struct name.
*/

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn rect_area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        username: String::from("someusername456"),
        active: true,
        sign_in_count: 1,
        email: String::from("someone@example.com"),
    };

    user2.email = String::from("anotheremail@example.com");

    let _user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let _user4 = User {
        email: String::from("another@example.com"),
        username: String::from("someusername789"), // This is required as user1.username and user1.active are already moved to _user3
        ..user1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    //The black and origin values are different types because they’re instances of different tuple structs.

    let _subject = AlwaysEqual;

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    println!("rect1 is {:#?}", rect1);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.rect_area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let _sq = Rectangle::square(3);
}

fn build_new_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
