/// @Author: Mitul
/// @Date:   2024-11-15 20:05:04
/// @Description: Common Programming Conecpts
/*
****************************************************************Keywords**************************************************************************************
- Keywords: The Rust language has a set of keywords that are reserved for use by the language only, much as in other languages. These words can not be used as
  names of variables or functions. Most of the keywords have special meanings and will be used to do various tasks in the Rust programs; a few have no current
  functionality associated with them but have been reserved for functionality that might be added to Rust in the future.
- The following is a list of keywords currently in use, with their functionality described.
    as - perform primitive casting, disambiguate the specific trait containing an item, or rename items in use statements
    async - return a Future instead of blocking the current thread
    await - suspend execution until the result of a Future is ready
    break - exit a loop immediately
    const - define constant items or constant raw pointers
    continue - continue to the next loop iteration
    crate - in a module path, refers to the crate root
    dyn - dynamic dispatch to a trait object
    else - fallback for if and if let control flow constructs
    enum - define an enumeration
    extern - link an external function or variable
    false - Boolean false literal
    fn - define a function or the function pointer type
    for - loop over items from an iterator, implement a trait, or specify a higher-ranked lifetime
    if - branch based on the result of a conditional expression
    impl - implement inherent or trait functionality
    in - part of for loop syntax
    let - bind a variable
    loop - loop unconditionally
    match - match a value to patterns
    mod - define a module
    move - make a closure take ownership of all its captures
    mut - denote mutability in references, raw pointers, or pattern bindings
    pub - denote public visibility in struct fields, impl blocks, or modules
    ref - bind by reference
    return - return from function
    Self - a type alias for the type we are defining or implementing
    self - method subject or current module
    static - global variable or lifetime lasting the entire program execution
    struct - define a structure
    super - parent module of the current module
    trait - define a trait
    true - Boolean true literal
    type - define a type alias or associated type
    union - define a union; is only a keyword when used in a union declaration
    unsafe - denote unsafe code, functions, traits, or implementations
    use - bring symbols into scope
    where - denote clauses that constrain a type
    while - loop conditionally based on the result of an expression
- Keywords Reserved for Future Use- The following keywords do not yet have any functionality but are reserved by Rust for potential future use:
  abstract, become, box, do, final, macro, override, priv, try, typeof, unsized, virtual, yield
- Raw identifiers are the syntax that lets code use keywords where they wouldn’t normally be allowed. To use a raw identifier prefix the keyword with r#.

***************************************************************Variables**************************************************************************************
- Variables in Rust are defined through the let keyword. In Rust, variables are immutable by default, meaning once the variable is assigned a value, the value
  won’t change. To make a variable mutable, mut keyword is added before the variable name. This allows the variable to be mutated. A variable can be used only
  if it has been initialized.
- // denotes the start of the comment.
- Multiple let expressions can define multiple variables with the same name, known as variable shadowing. Variable shadowing allows transforming variables
  without having to name the variables differently. Variable shadowing is also possible for values of different types.Shadowing means that the first variable is
  shadowed by the second, which means that the second variable is what the compiler will see when the code uses the name of the variable. In effect, the second
  variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.
- In Rust, blocks of code are delimited by curly brackets. The {} define the scope. A scope is the range within the program for which the item is valid.
- Putting '_' infront of an unused variable can suppress the unused variable warning. Also the macro can tell the compiler to ignore the warning.
- Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants
  and variables. The keyword mut is not allowed with constants. Constants aren’t just immutable by default—they’re always immutable. They can be declared using
  the const keyword instead of let, and the type of the value must be annotated. Constants can be declared in any scope, including the global scope, which makes
  them useful for values that many parts of code need to know about. The last difference is that constants may be set only to a constant expression, not the
  result of a value that could only be computed at runtime. Constants are valid for the entire time a program runs, within the scope in which they were declared.
- Rust’s naming convention for constants is to use all uppercase with underscores between words.

****************************************************************Data Types**************************************************************************************
- Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data.  Rust is a
  statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type to use based on
  the value and how it is used. In cases when many types are possible, a type annotation is added.
- Rust provides access to a wide variety of primitives. A sample includes:
    -> Scalar Types
        - Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
        - Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
        - Floating point: f32, f64
        - char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
        - bool either true or false
        - The unit type (), whose only possible value is an empty tuple: ()
    -> Compound Types
        - Arrays like [1, 2, 3]
        - Tuples like (1, true)
- Variables can always be type annotated. Numbers may additionally be annotated via a suffix or by default. Integers default to i32 and floats to f64.
- A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
- An integer is a number without a fractional component. The built in integer types are: i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize and usize.
  Each variant can be either signed or unsigned and has an explicit size. Signed and unsigned refer to whether it’s possible for the number to be negative—
  in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a
  sign (unsigned). Signed numbers are stored using two’s complement representation. Each signed variant can store numbers from -(2^n - 1) to (2^(n - 1) - 1)
  inclusive, where n is the number of bits that variant uses. Unsigned variants can store numbers from 0 to 2^n - 1. Additionally, the isize and usize types
  depend on the architecture of the computer on which the program is running on. Integer type defaults to i32. The primary situation in which code can use
  isize or usize is when indexing some sort of collection.
- Integers can, alternatively, be expressed using hexadecimal, octal or binary notation using these prefixes respectively: 0x, 0o or 0b.
- Number literals can be written in different forms: 98_222, 0xfff, 0o77, 0b1111_0000 and b'A' (Byte- u8 only). Number literals use _ as a visual separator to make
  the number easier to read. The number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type.
- When compiling in debug mode, Rust includes checks for integer overflow that can cause program to panic at runtime if this behavior occurs. When compiling
  in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs
  two’s complement wrapping. In short, values greater than the maximum value the type can hold "wrap around" to the minimum of the values the type can hold.
- Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are
  32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more
  precision. All floating-point types are signed. Rust also supports scientific E-notation, e.g. 1e6, 7.6e-4. The associated type is f64.
- Rust supports the basic mathematical operations for all the number types: addition, subtraction, multiplication, division, and remainder. Integer division
  truncates toward zero to the nearest integer. The operators available and their precedence in Rust are similar to other C-like languages.
- A Boolean type in Rust has two possible values: true and false. Booleans are one byte in size. The Boolean type in Rust is specified using bool. The main
  way to use Boolean values is through conditionals, such as an if expression.
- Rust’s char type is the language’s most primitive alphabetic type. The char literals are specified with single quotes, as opposed to string literals, which
  use double quotes. Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
  Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range from
  U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
- Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
- A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared,
  they cannot grow or shrink in size.A tuple is created by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type,
  and the types of the different values in the tuple don’t have to be the same. Optional type annotations can be added.
- A tuple is a collection of values of different types. Tuples are constructed using parentheses (), and each tuple itself is a value with type signature
  (T1, T2, ...), where T1, T2 are the types of its members. Functions can use tuples to return multiple values, as tuples can hold any number of values.
- Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in
  some other languages, arrays in Rust have a fixed length. Arrays are useful when data is to be allocated on the stack rather than the heap or when the number
  of elements is fixed. An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed
  to grow or shrink in size.
- An array’s type can be written using square brackets with the type of each element, a semicolon, and then the number of elements in the array. An array can be
  initialized to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square
  brackets.
- An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. To access elements of an array indexing is used.
- If invalid location is accessed then the Rust protects against this by immediately exiting instead of allowing the memory access and continuing.

****************************************************************Type Casting**************************************************************************************
- Rust is a statically and strongly typed language. Operations of two different types can result in overflow or other errors.
- Type Casting allows to convert a type from one to other. Explicit type conversion is required when going from lower to higher types. An overflow make occur
  when going from higher to lower.
*/
use std::io;

#[allow(unused_variables)]
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

// Tuples can be used as function arguments and as return values.
fn _reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

fn main() {
    println!("Hello, world!");
    assert!(r#match("foo", "foobar"));

    let apples = 5; // immutable
    let mut bananas = 5; //mutable
    bananas += 10;
    println!("There are {apples} apples and {bananas} bananas.");

    // Variable Shadowing
    let foo = 10;
    println!("The value of foo is {foo}");
    let foo = foo * 2;
    println!("The value of foo is {foo}");
    let _spaces = "   ";
    let _spaces = _spaces.len();
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Data Types
    let _guess: u32 = "42".parse().expect("Not a number!"); // Type Annotation
    let _x = 2.0; // f64

    let _a: i32 = 98_222; // Decimal
    let _b: i32 = 0xff; // Hex
    let _c: i32 = 0o77; // Octal
    let _d: i32 = 0b1111_0000; // Binary
    let _e: u8  = b'A';  // Byte (u8 only)
    let _f: u8  = 255;

    let _y: f32 = 3.0; // f32
    let _three_hours: u32 = THREE_HOURS_IN_SECONDS;

    // Mathematical Operations
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    // Boolean Type
    let _t = true;

    let _f: bool = false; // with explicit type annotation
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // Compound Types
    // Tuple
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    //This first creates a tuple and binds it to the variable tup. It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z.
    //this is called destructuring because it breaks the single tuple into three parts.
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of (x,y,z) is: {}{}{}",x,y,z);
    // A tuple with a bunch of different types.
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    // But long Tuples (more than 12 elements) cannot be printed.
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    // Arrays
    let _a = [1, 2, 3, 4, 5];
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _arr = [3; 5];
    let _first = a[0];
    let _second = a[1];

    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let _ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    // TODO: Resolve the dependency erro.
    // println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Type Casting
    let x = 255u8;
    let y = 34_i64;
    let z = 45.56 as f64;

    let sum = (x as u64) + (y as u64);
    let div = z / (y as f64);
    println!("Operation output {} and {}", sum, div);

    let x = (i32::MAX as i64) + 1;
    let y = 10_i32;

    let z = (x as i32) / y; // Case of overflow
    println!("Z = {}", z);

    // String to Integer Conversion
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Expected to read line");

    println!("{}", input);
    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input + 2);
}
