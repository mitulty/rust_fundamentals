/// @Author: Mitul
/// @Date:   2024-11-15 20:05:04
/// @Description: Managing Growing Projects
/*
- By default there is a binary crate with the same name as package is created if there is a file named main.rs in src directory and main.rs will be the crate root.
- Crate root is a source file that the rust compiler starts at when building the crate. It is also makes up the root module of the crate.
- If there is a lib.rs in the src directory then the rust will automatically create a library crate with the same name as the package.
- A package should have atleast one crate. It can have atmost one library crate and zero or more binary crates. To create more binary crates a bin folder is creatd
  in the src directory and define multiple crates.
- To create a binary crate use --bin and --lib for a library crate with cargo new.
*/
fn main() {
    println!("Hello, world!");
}
