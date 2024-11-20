/// @Author: Mitul
/// @Date:   2024-11-15 20:05:04
/// @Description: Cargo Tool
/*
- Cargo is Rust’s build system and package manager.  Cargo handles a lot of tasks, such as building the code, downloading the libraries the code depends on, 
  and building those libraries. The libraries that the code needs are called dependencies. Cargo comes installed with Rust
- The command 'cargo --version' gives the cargo version.
- To create a new project using Cargo, the command 'cargo new <project_name>' is executed. With this command, the Cargo generates two files and one directory:
  a Cargo.toml file and a src directory with a main.rs file inside.
- It also initializes a new Git repository along with a .gitignore file. Git files won’t be generated if user run cargo new within an existing Git repository; 
  user can override this behavior by using 'cargo new --vcs=git'.
- The file in the TOML (Tom’s Obvious, Minimal Language) format is Cargo’s configuration format. The [package] line is a section heading that indicates that 
  the following statements are configuring a package. The next three lines set the configuration information Cargo needs to compile the program: the name, 
  the version, and the edition of Rust to use. The line, [dependencies] is the start of a section for user to list any of the project’s dependencies. 
- In Rust, packages of code are referred to as crates.
- To build the prject the command 'cargo build' is executed. This command creates an executable file in target/debug/project_name.
- The command 'cargo run' can be used to compile the code and then run the resultant executable all in one command.
- Cargo also provides a command called 'cargo check'. This command quickly checks the code to make sure it compiles but doesn’t produce an executable. The 
  cargo check is much faster than cargo build because it skips the step of producing an executable.
- The command 'cargo build --release' can be used to compile the project with optimizations. This command will create an executable in target/release instead
  of target/debug. The optimizations make the Rust code run faster, but turning them on lengthens the time it takes for the program to compile. This is why 
  there are two different profiles: one for development, when user want to rebuild quickly and often, and another for building the final program.
- With simple projects, Cargo doesn’t provide a lot of value over just using rustc, but it will prove its worth as your programs become more intricate. 
  Once programs grow to multiple files or need a dependency, it’s much easier to let Cargo coordinate the build.
- Summary of Cargo and commands:
    - Create a project using 'cargo new <project_name>'.
    - Build a project using 'cargo build'. Use --release to build with optimizations.
    - Build and run a project in one step using 'cargo run'. Use --release to run with optimizations.
    - Build a project without producing a binary to check for errors using 'cargo check'.
    - Instead of saving the result of the build in the same directory as the code, Cargo stores it in the target/debug directory.
*/