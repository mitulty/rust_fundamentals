/// @Author: Mitul
/// @Date:   2024-11-15 20:05:04
/// @Description: Multiple Modules in different files
/*
- Rust also supports an older style of file path. For a module named front_of_house declared in the crate root, the compiler will look for the module’s code in:
    -> src/front_of_house.rs
    -> src/front_of_house/mod.rs (older style, still supported path)
For a module named hosting that is a submodule of front_of_house, the compiler will look for the module’s code in:
    -> src/front_of_house/hosting.rs
    -> src/front_of_house/hosting/mod.rs (older style, still supported path)
*/
pub use nested_module;
