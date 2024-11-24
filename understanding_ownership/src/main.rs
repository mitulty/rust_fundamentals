/// @Author: Mitul
/// @Date:   2024-11-15 20:05:04
/// @Description: Concept of Ownership
/*
- Ownership enables Rust to make memory safety guarantees without needing a garbage collector.
- Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some
  languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate
  and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules
  are violated, the program won’t compile. None of the features of ownership will slow down program while it’s running.
- Both the stack and the heap are parts of memory available to the code to use at runtime, but they are structured in different ways. The stack stores values in the
  order it gets them and removes the values in the opposite order. This is referred to as last in, first out.  Adding data is called pushing onto the stack, and
  removing data is called popping off the stack. All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size
  that might change must be stored on the heap instead.
- The heap is less organized. When a data is put on the heap, the code requests a certain amount of space. The memory allocator finds an empty spot in the heap that
  is big enough, mark it as being in used, and returns a pointer, which is the address of that location. This process is called allocating on the heap. Because the
  pointer to the heap is a known, fixed size, code can store the pointer on the stack, but when code wants the actual data, it must follow the pointer.
- Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the
  top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and
  then perform bookkeeping to prepare for the next allocation. Accessing data in the heap is slower than accessing data on the stack because code has to follow a
  pointer to get there. Contemporary processors are faster if they jump around less in memory.A processor can do its job better if it works on data that’s close to
  other data (as it is on the stack) rather than farther away (as it can be on the heap).
- When the code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get
  pushed onto the stack. When the function is over, those values get popped off the stack.
- Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap
  so that the code does not run out of space are all problems that ownership addresses.
- The main purpose of ownership is to manage heap data.
- A scope is the range within a program for which an item is valid.
********************************************************************Memory and Allocation*****************************************************************************
- In the case of a string literal, user knows the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals
  are fast and efficient. But these properties only come from the string literal’s immutability. Unfortunately, user can’t put a blob of memory into the binary for
  each piece of text whose size is unknown at compile time and whose size might change while running the program. With the String type, in order to support a mutable,
  growable piece of text, user needs to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means: The memory must be
  requested from the memory allocator at runtime and user needs a way of returning this memory to the allocator when done with the String. That first part is done by
  code: when user calls String::from, its implementation requests the memory it needs. This is pretty much universal in programming languages.
- However, the second part is different. In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, and user
  doesn’t need to think about it. In most languages without a GC, it’s the code's responsibility to identify when memory is no longer being used and call free method
  explicitly, just as done to request it. Doing this correctly has historically been a difficult programming problem. If the user forgets, memory will be wasted. If
  the code does it too early, it will have an invalid variable. If the code does it twice, that’s a bug too. The code needs to pair exactly one allocate with exactly
  one free.
- Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. When a variable goes out of scope, Rust calls a
  special function. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the
  closing curly bracket.
********************************************************************Ownership Rules***********************************************************************************
- Each value in a Rust has a variable that's called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
********************************************************************Copy Trait****************************************************************************************
- Rust has a special annotation called the Copy trait that user can place on types that are stored on the stack, as integers are. If a type implements the Copy trait,
  variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
- Rust won’t let annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the
  value goes out of scope and user adds the Copy annotation to that type, this will result in a compile-time error. The clone trait will be required for such types.
- Some of the types that implement Copy:
    -> All the integer types, such as u32.
    -> The Boolean type, bool, with values true and false.
    -> All the floating-point types, such as f64.
    -> The character type, char.
    -> Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
********************************************************************Ownership and Functions**************************************************************************
- The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy,
  just as assignment does.
- Returning values can also transfer ownership.
- The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes
  out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
  ********************************************************************Referecnes and Rules****************************************************************************
- A reference is like a pointer, it’s an address the code can follow to access the data stored at that address; that data is owned by some other variable. Unlike a
  pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
- Rust has a feature for using a value without transferring ownership, called references.
- Just as variables are immutable by default, so are references. To modify a borrowed value a mutable reference is used.
- At any given time, there can be either one mutable reference or any number of immutable references.
- References must always be valid.
- Mutable references have one big restriction: if code has a mutable reference to a value, it can have no other references (mutable or immutable) to that value.
- The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these
  three behaviors occur:
    -> Two or more pointers access the same data at the same time.
    -> At least one of the pointers is being used to write to the data.
    -> There’s no mechanism being used to synchronize access to the data.
- Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem by
  refusing to compile code with data races!
- Rust enforces a similar rule for combining mutable and immutable references.
- Code also cannot have a mutable reference while it has an immutable one to the same value. Users of an immutable reference don’t expect the value to suddenly
  change out from under them! However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s
  reading of the data. A reference’s scope starts from where it is introduced and continues through the last time that reference is used.
********************************************************************Dangling Referecnes*****************************************************************************
- In languages with pointers, it’s easy to erroneously create a dangling pointer—a pointer that references a location in memory that may have been given to someone
  else—by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling
  references: if code has a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
********************************************************************The Slice Type**********************************************************************************
- Slices let code reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have
  ownership.
- A string slice is a reference to part of a String or a string literal. A string literal itself is a string slice reference.
*/
fn main() {
    // Concept of Ownership
    println!("Hello, world!");
    {
        // s is not valid here, it’s not yet declared
        let _s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    {
        // s is not valid here, it’s not yet declared
        let _s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    let x = 5;
    let _y = x;
    //In this case the value of x is copied to y as the size of x is known at the compile time and x and y are pused to the stack.
    //The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies
    //of the actual values are quick to make. That means there’s no reason compiler would want to prevent x from being valid after
    //the y variable is created. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t
    // do anything different from the usual shallow copying.

    let s1 = String::from("hello");
    //A String is made up of three parts: a pointer to the memory that holds the contents of the string, a length, and a capacity.
    //This group of data is stored on the stack. On the other hand is the memory on the heap that holds the contents.
    //The length is how much memory, in bytes, the contents of the String are currently using. The capacity is the total amount of memory, in bytes,
    //that the String has received from the allocator. The difference between length and capacity matters,

    let s2 = s1;
    //When code assigns s1 to s2, the String data is copied, meaning rust copies the pointer, the length, and the capacity that are on the stack.
    //It does not copy the data on the heap that the pointer refers to.  If Rust did this, the operation s2 = s1 could be very expensive in terms of runtime
    //performance if the data on the heap were large. Even if it does, there will be a problem when one of the variable goes out of scope the data will be
    //dropped immediately because the Rust calls the drop function and cleans up the heap memory for that varibale. The other variable will not able to access it.
    //Also when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs.
    //Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
    //To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes
    //out of scope.

    // println!("{s1}, world!"); // This will create an error.
    println!("{s2}, world!");

    //The concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also
    //invalidates the first variable, instead of being called a shallow copy, it’s known as a move. Here s1 was moved into s2.
    //With only s2 valid, when it goes out of scope it alone will free the memory.
    //In addition, there’s a design choice that’s implied by this: Rust will never automatically create "deep" copies of the data. Therefore, any automatic
    //copying can be assumed to be inexpensive in terms of runtime performance.

    //If the code wants to deeply copy the heap data of the String, not just the stack data, it can use a common method called clone.

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
    // This works just fine and explicitly produces the behavior where the heap data does get copied.

    // Ownership and Functions

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function and so is no longer valid here

    // println!("{s}, world!"); // This will create an error.

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    println!("x is {}", x);
    // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    let _s1 = gives_ownership(); // gives_ownership moves its return
                                 // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");

    //References
    let s1 = String::from("hello");

    let len = calculate_length_ref(&s1);
    //The scope in which the variable s is valid is the same as any function parameter’s scope, but the value pointed to by the reference is not dropped when s
    //stops being used, because s doesn’t have ownership. When functions have references as parameters instead of the actual values, it won’t need to return the
    //values in order to give back the ownership, because it never had ownership. This is called as creating a reference borrowing.

    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");

    change(&mut s);

    let mut _s = String::from("hello");

    // This will throw an error.
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so a new reference can be made.

    let _r2 = &mut s;

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    // The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created.
    // These scopes don’t overlap, so this code is allowed: the compiler can tell that the reference is no longer being used at a point before the end of the scope.

    // let reference_to_nothing = dangle();
    let _reference_to_nothing = no_dangle();

    // The Slice Type
    let _word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    let s = String::from("hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];

    let s = String::from("hello");

    // Both are same
    let _slice = &s[0..2];
    let _slice = &s[..2];

    let s = String::from("hello");

    let len = s.len();

    // Both are same
    let _slice = &s[3..len];
    let _slice = &s[3..];

    // Both are same
    let _slice = &s[0..len];
    let _slice = &s[..];

    let mut s = String::from("hello world");

    let _word = first_word_modified(&s);

    s.clear();

    // error!
    // println!("the first word is: {word}");

    let _s = "Hello, world!";
    //The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an
    //immutable reference.

    let my_string = String::from("hello world");

    // `first_word_advanced` works on slices of `String`s, whether partial or whole
    let _word = first_word_advanced(&my_string[0..6]);
    let _word = first_word_advanced(&my_string[..]);
    // `first_word_advanced` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word_advanced(&my_string);

    let my_string_literal = "hello world";

    // `first_word_advanced` works on slices of string literals, whether partial or whole
    let _word = first_word_advanced(&my_string_literal[0..6]);
    let _word = first_word_advanced(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word_advanced(my_string_literal);

    // Array slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. N

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}
// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_modified(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_advanced(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
