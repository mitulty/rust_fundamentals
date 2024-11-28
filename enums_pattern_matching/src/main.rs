/// @Author: Mitul
/// @Date:   2024-11-15 20:05:04
/// @Description: Structure
/*
- Enums allow to define a type by enumerating its possible variants. Enums give a way of saying a value is one of a possible set of values. An enum value can only be
  one of its variants.
- The variants of the enum are namespaced under its identifier. The enum variants can also store different types of data.
- Methods can be defined on enums.
- Option is another enum defined by the standard library. The Option type encodes the very common scenario in which a value could be something or it could be nothing.
- Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>, and it is defined by the
  standard library as follows:
                    enum Option<T> {
                        None,
                        Some(T),
                    }
- The Option<T> enum is so useful that it’s even included in the prelude; code need not to bring it into scope explicitly. Its variants are also included in the
  prelude. The code can use "Some" and "None" directly without the Option:: prefix. The Option<T> enum is still just a regular enum, and Some(T) and None are still
  variants of type Option<T>. The <T> syntax is a feature of Rust. It’s a generic type parameter.
- Option<T> and T (where T can be any type) are different types and the compiler won’t let use an Option<T> value as if it were definitely a valid value.
**************************************************************************Match Control Flow*************************************************************************
- Rust has an extremely powerful control flow construct called match that allows to compare a value against a series of patterns and then execute code based on which
  pattern matches. Patterns can be made up of literal values, variable names, wildcards, and many other things.
- The power of match comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.
- Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how code can extract values out of enum
  variants.
- The arms’ patterns must cover all possibilities. An arm has two parts: a pattern and some code. The first arm here has a pattern that is the value and then the =>
  operator that separates the pattern and the code to be run.
- Rust also has a pattern code can use when it wants a catch-all but don’t want to use the value in the catch-all pattern: _ is a special pattern that matches any
  value and does not bind to that value.
- When the match expression executes, it compares the resultant value against the pattern of each arm, in order. If a pattern matches the value, the code associated
  with that pattern is executed. If that pattern doesn’t match the value, execution continues to the next arm.
- The code associated with each arm is an expression, and the resultant value of the expression in the matching arm is the value that gets returned for the entire
  match expression.
- Curly brackets are not used if the match arm code is short. If multiple lines are to be run as a part of the code in a match arm, curly brackets must be used, and
  the comma following the arm is then optional.
- Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how code can extract values out of enum variants.
- The if let syntax lets combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest. The if let is syntax sugar for a
  match that runs code when the value matches one pattern and then ignores all other values.
- The syntax if let takes a pattern and an expression separated by an equal sign. It works the same way as a match, where the expression is given to the match and
  the pattern is its first arm.
- An else can be included with an if let. The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression
  that is equivalent to the if let and else.
*/
#[warn(unused_variables)]
enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // Named Tuples
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

enum IpAddrKindData {
    V4(String),
    V6(u16, u16, u16, u16, u16, u16, u16, u16),
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    California,
    Arizona,
}

#[derive(Debug)] // so we can inspect the state in a minute
enum CoinState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_state(coin: CoinState) -> u8 {
    match coin {
        CoinState::Penny => 1,
        CoinState::Nickel => 5,
        CoinState::Dime => 10,
        CoinState::Quarter(state) => {
            //When a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state.
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let _ipv4 = IpAddrKindData::V4(String::from("192.168.1.1"));
    let _ipv6 = IpAddrKindData::V6(
        0xFE80, 0xCD00, 0x0000, 0x0CDE, 0x1257, 0x00000, 0x211E, 0x729C,
    );
    let m = Message::Write(String::from("hello"));
    m.call();
    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;
    //Error
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
    let _sum = x + y.unwrap_or(0);

    value_in_cents_state(CoinState::Quarter(UsState::Arizona));

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // _ => reroll(),
        _ => (),
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;
    let coin = CoinState::Quarter(UsState::Alaska);
    match coin {
        CoinState::Quarter(UsState::Alaska) => {
            println!("State quarter from {coin:#?}!")
        }
        _ => count += 1,
    }

    let mut count = 0;
    let coin = CoinState::Quarter(UsState::Alabama);
    if let CoinState::Quarter(UsState::Alabama) = coin {
        println!("State quarter from {coin:#?}!");
    } else {
        count += 1;
    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn _reroll() {}
fn _route(_ip_kind: IpAddrKind) {}
