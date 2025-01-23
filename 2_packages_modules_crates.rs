/// @Author: Mitul Tyagi
/// @Date:   2024-11-28 22:26:10
/// @Description: Concept of Packaes, Modules and Crates
/*
- Rust has a number of features that allow to manage code’s organization, including which details are exposed, which details are private, and what names are in each 
  scope in the program. These features, sometimes collectively referred to as the module system, include:
    -> Packages: A Cargo feature that lets you build, test, and share crates
    -> Crates: A tree of modules that produces a library or executable
    -> Modules and use: Let you control the organization, scope, and privacy of paths
    -> Paths: A way of naming an item, such as a struct, function, or module
- A crate is the smallest amount of code that the Rust compiler considers at a time. Even if rustc is run rather than cargo and pass a single source code file the 
  compiler considers that file to be a crate. Crates can contain modules, and the modules may be defined in other files that get compiled with the crate. Crates can 
  contain modules, and the modules may be defined in other files that get compiled with the crate.
- A crate can come in one of two forms: a binary crate or a library crate. Binary crates are programs that are compiled to an executable that can be run, such as a 
  command-line program or a server. Each must have a function called main that defines what happens when the executable runs. 
- Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects.
- Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library”.
- The crate root is a source file that the Rust compiler starts from and makes up the root module of the crate.
- A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates. 
  Cargo is actually a package that contains the binary crate for the command-line tool.
- The Cargo package also contains a library crate that the binary crate depends on. Other projects can depend on the Cargo library crate to use the same logic the 
  Cargo command-line tool uses. A package can contain as many binary crates, but at most only one library crate. A package must contain at least one crate, whether 
  that’s a library or binary crate.
- Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package 
  directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes the crate 
  root files to rustc to build the library or binary.
- If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package. A package can have multiple 
  binary crates by placing files in the src/bin directory: each file will be a separate binary crate.
****************************************************************************Working of Modules***********************************************************************
- Start from the crate root: When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a 
  binary crate) for code to compile.
- Declaring modules: In the crate root file new modules can be decalred with mod keyword.
- Declaring submodules: In any file other than the crate root, the code can declare submodules in the main module.
- Paths to code in modules: Once a module is part of the crate, the code in that module can be referred from anywhere else in that same crate, as long as the privacy
  rules allow, using the path to the code.
- Private vs. public: Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod. To make 
  items within a public module public as well, use pub before their declarations.
- The use keyword: Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths.
- Modules let organize code within a crate for readability and easy reuse. Modules also allow to control the privacy of items because code within a module is private 
  by default. Private items are internal implementation details not available for outside use. Code can choose to make modules and the items within them public, which
  exposes them to allow external code to use and depend on them.
- To show Rust where to find an item in a module tree, paths are used. A path can take two forms:
    -> An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from
       the current crate, it starts with the literal crate.
    -> A relative path starts from the current module and uses self, super, or an identifier in the current module.
- Both absolute and relative paths are followed by one or more identifiers separated by double colons (::). The pub keyword on a module only lets code in its ancestor
  modules refer to it, not access its inner code.
- a package can contain both a src/main.rs binary crate root as well as a src/lib.rs library crate root, and both crates will have the package name by default. 
  Typically, packages with this pattern of containing both a library and a binary crate will have just enough code in the binary crate to start an executable that 
  calls code within the library crate. This lets other projects benefit from most of the functionality that the package provides because the library crate’s code can
  be shared. 
- The module tree should be defined in src/lib.rs. Then, any public items can be used in the binary crate by starting paths with the name of the package. The binary 
  crate becomes a user of the library crate just like a completely external crate would use the library crate: it can only use the public API. 
- The pub can be used to designate structs and enums as public, but there are a few extra details to the usage of pub with structs and enums. If pub is put before a 
  struct definition, only the struct is made public, but the struct’s fields will still be private. Each field can be made public or not on a case-by-case basis.
- Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for 
  enum variants is to be public. Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by 
  default unless annotated with pub.
- Having to write out the paths to call functions can feel inconvenient and repetitive. A shortcut can be created to a path with the use keyword once, and then the 
  shorter name is used everywhere else in the scope. Adding use and a path in a scope is similar to creating a symbolic link in the filesystem.
- To bringpu two types of the same name into the same scope with use: after the path, cide can specify as and a new local name, or alias, for the type.
- When code brings a name into scope with the use keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that 
  name as if it had been defined in that code’s scope, pub and use can be combined. This technique is called re-exporting because we’re bringing an item into scope 
  but also making that item available for others to bring into their scope.
- Members of the Rust community have made many packages available at crates.io, and pulling any of them into the package involves these same steps: listing them in 
  the package’s Cargo.toml file and using use to bring items from their crates into scope.
- To bring all public items defined in a path into scope, that path can be spcified and then followed by the * glob operator.
*/