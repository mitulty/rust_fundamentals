/// @Author: Mitul
/// @Date:   2024-11-15 20:05:04
/// @Description: Functions and Control Flow
/*
****************************************************************Functions*************************************************************************************
- Functions are prevalent in Rust code. The main function is the entry point of many programs.
- Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.
- A function is defined in Rust by entering fn followed by a function name and a set of parentheses. The curly brackets tell the compiler where the function
  body begins and ends. A function can be called by entering its name followed by a set of paranthesis.
- Rust doesn’t care where the code defines the functions, only that they’re defined somewhere in a scope that can be seen by the caller.
- Functions can be defined to have parameters, which are special variables that are part of a function’s signature. When a function has parameters, it can be
  provide with concrete values for those parameters. Technically, the concrete values are called arguments. In function signatures, the type of each parameter
  has to be declared explicitly.
- Function bodies are made up of a series of statements optionally ending in an expression.
- Statements are instructions that perform some action and do not return a value. For example: creating a variable and assigning a value to it with the let
  keyword is a statement. Function definitions are also statements. Statements do not return values.
- Expressions evaluate to a resultant value. Expressions can be part of statements. Calling a function is an expression. Calling a macro is an expression. A
  new scope block created with curly brackets is an expression.
- The expression:
    let y = {
                let x = 3;
                x + 1
            }
  is a block that evaluates to 4. The value gets bound to y as part of the let statement. The x + 1 line doesn’t have a semicolon at the end.
  Expressions do not include ending semicolons. If a semicolon is added to the end of an expression,it becomes a statement, and it will then not return a value.
- Functions can return value to the code that calls them. It's type has to be declared after an arrow instead of the name.

*****************************************************************If/Else If/Else*******************************************************************************
- The ability to run some code depending on whether a condition is true and to run some code repeatedly while a condition is true are basic building blocks in
  most programming languages. The most common constructs that let the code control the flow of execution are if expressions and loops.
- An if expression allows to branch the code depending on conditions. All if expressions start with the keyword if, followed by a condition. Blocks of code
  associated with the conditions in if expressions are sometimes called arms. Optionally, an else expression can be include to give the program an alternative
  block of code to execute should the condition evaluate to false. If an else expression is mentioned and the condition is false, the program will just skip the
  if block and move on to the next bit of code.
- Rust will not automatically try to convert non-Boolean types to a Boolean.The code must be explicit and always provide if with a Boolean as its condition.
- Multiple conditions and arms can be used by combining if and else in an else if expression.
- Because if is an expression, it can be used on the right side of a let statement to assign the outcome to a variable provided the arms have similar expression
  types.

*********************************************************************Loop**************************************************************************************
- It’s often useful to execute a block of code more than once. For this task, Rust provides several loops, which will run through the code inside the loop body
  to the end and then start immediately back at the beginning. Rust has three kinds of loops: loop, while, and for.
- The loop keyword tells Rust to execute a block of code over and over again forever or until told explicitly it to stop. Rust also provides a way to break out
  of a loop using code. The break keyword can be placed within the loop to tell the program when to stop executing the loop. The continue in a loop tells the
  program to skip over any remaining code in this iteration of the loop and go to the next iteration.
- Code can also use return from inside a loop. While break only exits the current loop, return always exits the current function.
- If there are nested loops, break and continue apply to the innermost loop at that point.A loop label can be optionally applied on a loop that can then be used
  with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote.
- A program will often need to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program
  calls break, stopping the loop. It’s possible to implement behavior like this using a combination of loop, if, else, and break. However, this pattern is so
  common that Rust has a built-in language construct for it, called a while loop. This construct eliminates a lot of nesting that would be necessary when used
  loop, if, else, and break. While a condition evaluates to true, the code runs; otherwise, it exits the loop.
- The while construct can be used to loop over the elements of a collection, such as an array. This approach is error prone though.
- As a more concise alternative, a for loop can be used to execute some code for each item in a collection.
- A semicolon at the end of the loop body makes it an expression which returns a value.
  */
fn main() {
    // Functions
    println!("Hello, world!");

    another_function();
    another_function_with_param(5);
    print_labeled_measurement(5, 'h');

    //  let x = (let y = 6); // This will be an error as inner let does not return anything.
    //This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment.

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");

    // Control Flow
    // If Expression
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Loops
    // loop {
    //     println!("again!");
    // }
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //  This will return a value of counter*2 (20)
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    let returned_value = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up 43;
            }
            remaining -= 1;
        }

        count += 1;
    };
    println!(
        "End count = {count} and Returned Value = {:#?}",
        returned_value
    );

    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_param(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
