/// @Author: Mitul
/// @Date:   2024-11-15 20:05:04
/// @Description: Nth Fibonacci Number
fn main() {
    let n = 15;
    let mut i = 2;
    let mut first = 0;
    let mut second = 1;
    let mut number;
    let fibonacci_number = loop {
        number = first + second;
        first = second;
        second = number;
        i += 1;
        if i > n {
            break number;
        }
    };
    println!("{}th Fibonacci Number is {:?}", n, fibonacci_number);
}
