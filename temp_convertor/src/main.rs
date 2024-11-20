/// @Author: Mitul
/// @Date:   2024-11-15 20:05:04
/// @Description: Temperature Conversion b/w Fahrenheit and Celcius
fn main() {
    let mut f_temp: i32 = 132;
    let mut cel_temp: i32;

    cel_temp = (f_temp - 32) * (5 / 9);
    println!("Celcius for {:?}F is {:?}C", f_temp, cel_temp);
}
