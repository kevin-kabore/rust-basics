// #region Here for dragions
trait TypeDebug {
    fn print_type(&self);
}

impl<T> TypeDebug for T {
    fn print_type(&self) {
        println!("The type is '{}'", std::any::type_name::<T>());
    }
}

// # endregion

fn main() {
    // mutable variables
    println!("--- Mutable variables");
    let mut x: i32 = 21; // mut = mutable, i32 = Type: integer 32 bits
    println!("The value of x is: {}", x);
    x = 5;
    println!("The new value of x is: {}", x);

    println!("--- Types");
    x.print_type();
    let y = 3.14;
    y.print_type();  // f64 = Type: float 64 bits

    let a = true;
    a.print_type(); // bool = Type: boolean
    let b: bool = false;
    if a { 
        println!("The boolean a is set to true")
    } else {
        println!("The boolean a is set to false")
    }
    if b {
        println!("The boolean b is set to true")
    } else {
        println!("The boolean b is set to false")
    }

    // Tuples: group variables of different types together
    println!("--- Tuples: group variables of different types together");
    let tup: (i32, &str, f64, &mut bool) = (500, "Hello World", 3.14, &mut false);
    println!("The tuple is: {:?}", tup);
    println!("The first two values of the tuple are: {} and {}", tup.0, tup.1);
    // grabbing values from a tuple: 2 options
    // option 1, variable assignment
    let var_one = tup.0;
    let var_two = tup.1;
    println!("The first two values of the tuple are: {} and {}", var_one, var_two);
    // option 2, destructuring: underscore to ignore values
    let (var_1, var_2, _, _) = tup;
    println!("The first two values of the tuple are: {} and {}", var_1, var_2);

    // Arrays: fixed size set of the same type of values
    println!("--- Arrays: fixed size set of the same type of values");
    let array_1 = [1,2,3];
    println!("The first value of the array is: {}", array_1[0]);

    // fill an array with repeating value. To make mutable, add mut before array
    let mut array_of_10_zeros: [i32; 10] = [0; 10]; // 0, 10 times. type = i32 of size 10
    println!("The array is : {:?}", array_of_10_zeros); // {:?} = debug format directive
    array_of_10_zeros[0] = 1;
    println!("The updated array is : {:?}", array_of_10_zeros);

    // Control flow
    println!("--- Control flow");
    let mut counter = 0;
    while counter <= 6 {
        if counter < 5 {
            println!("The counter is less than 5");
        } else if counter == 5{
            println!("The counter is equal to 5, you win!");
        } else {
            println!("The counter is greater than 5. The end.");
        }
        counter += 1;
    }

    // Loops
    println!("--- Loops");
    // loop and multiply x by 2 until it is greater than 2000
    let mut x = 1;
    println!("----- regular loop");
    loop {
        x *= 2;
        if x > 2000 {
            break;
        }
        println!("The value of x is: {}", x);
    }
    println!("Outside the loop, the value of x reached: {}", x);
    println!("----- while: execute a block of code while a condition is true");
    let mut y = 1;
    while y <= 2000 {
        y *= 2;
        println!("The value of y is: {}", y);
    }
    println!("Outside the while loop, the value of y reached: {}", y);

    // for loop
    println!("----- for loop: iterate over a collection or range");
    for x in 1..10 { // from 0 - 9
        println!("The value of x in EXclusive range: {}", x);
    }
    // inclusive range
    for y in 1..=10 { // from 1 - 10
        println!("The value of y in INClusive range: {}", y);
    }
    // iterators
    let array_2 = [1,2,3,4,5];
    for val in array_2 {
        println!("The value at this index is: {}", val);
    }
    
    // match statement
    println!("----- match statement: match a value against a number of patterns");
    let x = 1;
    match x {
        1 => println!("x is equal to 1"),
        2 => println!("x is equal to 2"),
        _ => println!("x is not equal to 1 or 2"), // _ = wildcard/catch-all
    }
    // match with a range
    println!("----- match with a range");
    let x = 5;
    match x {
        1..=5 => println!("x is in the range 1..=5"),
        _ => println!("x is not in the range 1..=5"),
    }
    // match with a tuple
    println!("----- match with a tuple & exhaustive match");
    let a = true;
    let b = false;
    match (a, b) {
        (true, false) => println!("a is true and b is false"),
        (false, true) => println!("a is false and b is true"),
        (true, true) => println!("a is true and b is true"),
        _ => println!("a is false and b is false"),
    }
}
