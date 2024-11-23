/// @Author: Mitul
/// @Date:   2024-11-15 20:05:04
/// @Description: Memory Management
/*
- Two important RAM memory segments are heap and stack that the program uses to handle data. Stack is Last in First out data storage.
- Data in stack remains as long as the scope exists. Outside the scope, the data will not be avaialable in stack. Stack is only capable of holding information
  that has a known and fixed size.
- Heap is used to store dynamically created data whose size is not known at the compile time.
- A string literal is constant which can be changed and remains in the stack. A string can be created dynamically in the heap using the String module.
*/
fn main() {
    println!("Hello, world!");
    let z = 2;
    let y = z;
    let string = String::from("Heap_Data");
    println!("y = {}", y);
    println!("{}", string);
}
