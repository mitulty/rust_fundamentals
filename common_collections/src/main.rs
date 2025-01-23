/// @Author: Mitul
/// @Date:   2024-11-15 20:05:04
/// @Description: Common Collections
/*
- Rust’s standard library includes a number of very useful data structures called collections. Most other data types represent one specific value, but collections
  can contain multiple values. Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data
  does not need to be known at compile time and can grow or shrink as the program runs.
- Some of the common collections are:
    -> A vector allows you to store a variable number of values next to each other.
    -> A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
    -> A hash map allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.
- Vectors allow to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the
  same type. To create a new empty vector, the Vec::new function is called. Vectors are implemented using generics. The Vec<T> type provided by the standard library
  can hold any type. When a vector is created to hold a specific type, the type can be specified within angle brackets.
- Rust conveniently provides the vec! macro, which will create a new vector that holds the values given to it. This does not need the type annotation.
- To create a vector and then add elements to it, the push method is used. There are two ways to reference a value stored in a vector: via indexing or by using the
  get method. Vectors are indexed by number, starting at zero. Using & and [] gives us a reference to the element at the index value. When the get method is used with
  the index passed as an argument, an Option<&T> is obtained that can be used with match.
- Rust provides two ways to reference an element when the code tries to use an index value outside the range of existing elements.
- When the get method is passed an index that is outside the vector, it returns None without panicking.When accessed using [] it will cause the program to panic.
- When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure this reference and any other references to the
  contents of the vector remain valid.
- To access each element in a vector in turn, code would iterate through all of the elements rather than use indices to access one at a time.
- To change the value that the mutable reference refers to, the * dereference operator is used. Iterating over a vector, whether immutably or mutably, is safe
  because of the borrow checker’s rules.
- Vectors can only store values that are of the same type. This can be inconvenient; there are definitely use cases for needing to store a list of items of different
  types. Fortunately, the variants of an enum are defined under the same enum type, to represent elements of one type to different types, enums can be used.
- Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. If Rust
  allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the
  vector. Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled.
- Like any other struct, a vector is freed when it goes out of scope. When the vector gets dropped, all of its contents are also dropped, meaning the integers it
  holds will be cleaned up. The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.
**********************************************************************Strings****************************************************************************************
- Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str. The string slices are references
  to some UTF-8 encoded string data stored elsewhere. String literals are stored in the program’s binary and are therefore string slices.
- The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.
  String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.
- Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters.
- Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs, no matter what
  human language the data is in.
- Rust strings don’t support indexing.
- Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a
  grapheme cluster, or a string slice. The best way to operate on pieces of strings is to be explicit for either characters and bytes. The bytes method returns each
  raw byte.
********************************************************************Hash Map*****************************************************************************************
- The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into
  memory. Hash maps are useful when the code wants to look up data not by using an index, as with vectors, but by using a key that can be of any type.
- For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map
  will be the owner of those values.
- Although the number of key and value pairs is growable, each unique key can only have one value associated with it at a time.
- It’s common to check whether a particular key already exists in the hash map with a value and then to take the following actions: if the key does exist in the hash
  map, the existing value should remain the way it is; if the key doesn’t exist, insert it and a value for it.
- By default, HashMap uses a hashing function called SipHash that can provide resistance to denial-of-service (DoS) attacks involving hash tables
*/
use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i += 50;
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // String
    // They are stored as a collection of UTF-8 encoded bytes.
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");
    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");

    let s1 = String::from("hello");
    // let h = s1[0]; error

    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Hash Maps

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point/

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
