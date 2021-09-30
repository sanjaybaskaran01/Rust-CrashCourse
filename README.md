<h1 align="center"> Rust Crash Course </h1>
<p  align="center">
<a href="https://www.rust-lang.org/learn">
<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/2048px-Rust_programming_language_black_logo.svg.png" width=100 height=100/>
</a>
</p>
<h3 align="center"> <a href="https://www.youtube.com/watch?v=zF34dRivLOw&t=655s&ab_channel=TraversyMedia">
 Reference </a></h3>

<br>

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Main](#main)
- [Print](#print)
- [Variables](#variables)
- [Data Types](#data-types)
- [Strings](#strings)
- [Tuples](#tuples)
- [Arrays](#arrays)
- [Vectors](#vectors)
- [Conditionals](#conditionals)
- [Loops](#loops)
- [Functions](#functions)
- [Pointers & Reference](#pointers--reference)
- [Structs](#structs)
- [Enums](#enums)
- [Command Line Args](#command-line-args)

<br>

## Main

```rust
// mod print;
// mod vars;
// mod types;
// mod strings;
// mod tuples;
// mod arrays;
// mod vectors;
// mod conditionals;
// mod loops;
// mod functions;
// mod pointer_ref;
// mod structs;
// mod enums;
// mod cli;
mod input;
fn main() {
    // println!("Hello, world!");
    // print::run();
    // vars::run();
    // types::run();
    // strings::run();
    // tuples::run();
    // arrays::run();
    // vectors::run();
    // conditionals::run();
    // loops::run();
    // functions::run();
    // pointer_ref::run();
    // structs::run();
    // enums::run();
    // cli::run();
    input::run();
}
```
<br>

## Print

```rust
pub fn run() {
    // Print to console
    println!("Hello from print.rs");
    // Basic Formatting
    println!("{} is from {}", "Sanjay", "india");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to code!", "Sanjay", "india");
    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Sanjay",
        activity = "Football"
    );
}
```
<br>

## Variables

```rust
pub fn run() {
    let name = "Sanjay";
    let mut age = 19;
    println!("My name is {} and I am {}", name, age);
    age = age + 1;

    println!("My name is {} and I am {}", name, age);
    // Assign multiple characters
    let (my_name, my_age) = ("Sanjay", 19);
    println!("{} is {}", my_name, my_age)
}
```
<br>

## Data Types

```rust
pub fn run() {
    let z: i64 = 3123123123123123;
    let is_active: bool = false;

    let face = "\u{1F643}";
    println!("{:?}", (z, is_active, face));
}
```

<br>

## Strings

```rust
// Primitive strings
//String = Growable heap allocated data structure

pub fn run() {
    let mut hello = String::from("Hello ");
    // Get Length
    println!("length={}", hello.len());

    // Push char
    hello.push('W');

    // Pushing string
    hello.push_str("orld");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // is empty
    println!("is empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There!"));

    // Loop through string by whitespace

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    println!("{}", s.capacity());
}
```
<br>

## Tuples
```rust
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Sanjay", "india", 19);

    println!(
        "{} is from {} and {} is {}",
        person.0, person.1, person.0, person.2
    );
}
```
<br>

## Arrays
```rust
// Arrays fixed list where elements are of same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];
    println!("Before Change:\t{:?}", numbers);
    numbers[2] = 10;
    // Get single character
    println!("After Change:\t{:?}", numbers);
    println!("in index 2:{}", numbers[2]);

    // Get array length
    println!("Array length:{}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice : {:?}", slice);
    println!("{}", slice == numbers);
}
```

## Vectors

```rust
// Vectors are resizeable Arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    println!("Before Change:\t{:?}", numbers);
    // Re-assign Value
    numbers[2] = 10;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    // Get single character
    println!("After Change:\t{:?}", numbers);
    println!("in index 2:{}", numbers[2]);

    // Get Vector length
    println!("Vector length:{}", numbers.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice : {:?}", slice);

    // Loop through vector values
    for num in numbers.iter() {
        println!("Number:{}", num);
    }

    // Loop and mutate values
    for num in numbers.iter_mut() {
        *num *= 2;
    }
    println!("Numbers Vec:{:?}", numbers);
}
```

## Conditionals
```rust
// used to check the condition and act on the result

pub fn run() {
    let age: u8 = 22;
    let check_id: bool = true;
    let knows_person_of_age = true;

    // If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender:What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender:Get out!");
    } else {
        println!("Bartender:I need to see your ID");
    }

    //Shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of Age:{}", is_of_age)
}
```
<br>

## Loops
```rust
// Loops - to iterate until a condition is met

pub fn run() {
    let mut count = 1;

    // Infinite loop
    // loop{
    //     count+=1;
    //     println!("{}",count);
    //     if count==20{
    //         break;
    //     }
    // }

    // While loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz!")
        } else if count % 3 == 0 {
            println!("Fizz!");
        } else if count % 5 == 0 {
            println!("Buzz!");
        } else {
            println!("{}", count);
        }
        //Inc
        count += 1;
    }

    // for range loop
    // for num in 1..100 {
    //     if num%15==0{
    //     println!("FizzBuzz!")
    //     }else if num%3==0{
    //         println!("Fizz!");
    //     }else if num%5==0{
    //         println!("Buzz!");
    //     }else{
    //         println!("{}",num);
    //     }
    // }
}
``` 
<br>

## Functions
```rust
pub fn run() {
    greeting("Hello", "Sanjay");
    // Bind functions to variables
    let get_sum = add(5, 5);
    println!("Sum:{}", get_sum);

    // Closure
    let n3: i32 = 15;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum:{}", add_nums(10, 10));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {},nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
```
<br>

## Pointers & Reference
```rust
// Reference Pointers - Point to a resource in memory

pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let _arr2 = arr1;

    // Vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("Values: {:?}", (&vec1, vec2));
}
```

## Structs
```rust
// Structs - used to create custom data types

// Traditional struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue:u8,
// }

// Tuple struct
// struct Color(u8,u8,u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct person

    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // With Traditional Struct

    // let mut c = Color {
    //     red:255,
    //     green:0,
    //     blue:0,
    // };
    // c.red=200;
    // println!("Color: {} {} {}",c.red,c.green,c.blue);
    // with Tuple Struct

    // let mut c = Color(255,0,0);
    // c.0=200;
    // println!("Color: {} {} {}",c.0,c.1,c.2);

    let mut p = Person::new("Mary", "Doe");
    println!("Person {}", p.full_name());
    p.set_last_name("Williams");
    // println!("Person {} {}",p.first_name,p.last_name);
    println!("Person {}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
}
```
<br>

## Enums
```rust
// Enums are types which have a few definite value

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Avatar moving Up!"),
        Movement::Left => println!("Avatar moving Left!"),
        Movement::Down => println!("Avatar moving Down!"),
        Movement::Right => println!("Avatar moving Right!"),
    }
}
pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
```
<br>

## Command Line Args
```rust
use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Sanjay";
    let status = "active";
    // println!("Command: {}",command);

    if command == "hello" {
        println!("Hi {}, How are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command");
    }
}
```