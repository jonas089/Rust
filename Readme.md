# Cargo Commands

cargo new -> create cargo tree \
cargo build  -> compile cargo project \
cargo run -> compile and run cargo project \
cargo check -> check if code can be compiled ( faster than compiling )


# Chapter 4 - Ownership

## Stack and Heap

Both the Stack and the Heap are parts of memory available for the code to use at runtime.
Stack and Heap are structured in different ways.

1. The Stack takes values in the order it gets them and removes them in the opposite order.
(last in, first out)

Push on / Pop off the stack, means adding and removing data respectively.

2. The Heap is less organized. When data is put on the heap, you request a certain amount
of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as
being in use and returns a pointer, which is the address of that location. This process is called
allocating(on the heap)

When code calls a function, the values passed into the function and the function's local variables get pushed onto the stack.
When the function is over, those values get popped off the stack.

## Copies of Variables
```python
  let s1 = String::from("hello");
  let s2 = s1;
  println!("{}, world!", s1);
```
s1 is now invalid and s2 becomes the only reference to the data on the heap.

-----
```python
  let x = 5;
  let y = x;
  println!("x = {}, y = {}", x, y);
```
types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y.

-----

### "Copy"

All the integer types, such as u32.
The Boolean type, bool, with values true and false.
All the floating point types, such as f64.
The character type, char.
Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

## Returning a tuple ( multiple values )
```python
  fn main() {
      let s1 = String::from("hello");
      let (s2, len) = calculate_length(s1);
      println!("The length of '{}' is {}.", s2, len);
  }


  fn calculate_length(s: String) -> (String, usize) {
      let length = s.len(); // len() returns the length of a String
      (s, length)
  }
```
## References and Borrowing

Reference to an object as a parameter instead of taking ownership of the value of the object. / s1 "survives" the entire scope.

```python
  fn main() {
      let s1 = String::from("hello");
      let len = calculate_length(&s1);
      println!("The length of '{}' is {}.", s1, len);
  }



  fn calculate_length(s: &String) -> usize {
      s.len()
  }
```

### &String

The &s1 syntax refers to the value of s1, without overtaking ownership. => The value will not be dropped and is still owned by variable s1.

#### References can't be modified / References are immutable

### Mutable References

```python
  fn main() {
      let mut s = String::from("hello");
      change(&mut s);
  }



  fn change(some_string: &mut String) {
      some_string.push_str(", world");
  }
```

There can only be one mutable reference to a piece of data at a time.

There can not be a mutable and an immutable reference at the same time.

### Multiple Mutable References ( not simultaneous )

```python
  let mut s = String::from("hello");
  {
      let r1 = &mut s;
  } // r1 goes out of scope here, so we can make a new reference with no problems.
  let r2 = &mut s;
```

Another example on mutable and immutable references that is OK:
```python
  let mut s = String::from("hello");
  let r1 = &s; // no problem
  let r2 = &s; // no problem
  println!("{} and {}", r1, r2);
  // variables r1 and r2 will not be used after this point
  let r3 = &mut s; // no problem
  println!("{}", r3);
```

# Chapter 5 - Use of Structs to structure related data

## Declare a Struct

A Struct can be instantiated, the way it is instantiated doesn't look too different from a python class.

```python
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

## Create an instance of the Struct

```python
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
```

## Create multiple instances of a struct

```python
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

The ..user1 states, that every undefined field's value is set to have the same value as the field of the given instance (user1)

```python
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```
The email of user2 is different to that of user1, but username etc. are the same.

## Tuple structs
```python
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

## References and Lifetime (foreshadowing)
Keep in mind that a struct containing an &str Reference may become invalid as memory is cleared.
We'll get back to this in chapter 10 and learn about "Lifetime".

## {:?}, var
:? inside the curly brackets tells println! to use an output format called Debug. This would for exmple also print
the values of a struct ( which is useful for debugging ).
{:#?} prints output that is easier to read.

## dbg! macro
#[derive(Debug)]

```python
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```
dbg! returns ownership of the assigned value

## Method Syntax
To define a function within the context of a struct, start an impl block for the context:
```python
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
We use &self here, as we don't want to take ownership / we only want to read the data in the struct, not write to it.

Reason to use impl blocks:
"We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide"

# Chapter 6 - Enums and Pattern Matching

Enumerations (dt. Aufzählungen) allow you to define a type by enumerating its possible variants (e.g. IPV4 and IPV6 addresses).

## Defining an Enum
```python
enum IpAddrKind {
    V4,
    V6,
}
```
ipAddrKind is now a custom data type that we can use elsewhere in the code.

```python
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```
-> the name of each Enum variant also become a function that constructs an instance of the enum.
IpAddr::V4() is a function call that takes a String argument and returns an Instance of type IpAddr.

## Advantage of Enums over Structs

Example: IP - addresses.

If we want to instantiate an IPV4 address by splitting it up in 4 "u8" values, but instantiate / express V6 addresses in a string format, we can not do so with a single struct.

However, this can be achieved using an Enumerator:

```python
enum IpAddr {
    V4(u8, u8, u8, u8),
    /*u8: 8bit integer, unsigned(positive).*/
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

## Option Type
Only when we have an Option<i8>, Option<&str> (or whatever type of value we’re working with) do we have to worry about possibly not having a value
The Option type is used to handle a scenario where a value could be something or nothing.
" In short, because Option<T> and T (where T can be any type) are different types, the compiler won’t let us use an Option<T> value as if it were definitely a valid value. "
Example:
```python
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```
This won't compile, because y has a different type to x and the compiler won't compute different types in a calculation like this.

Before we can use the Option type value, we need to handle the type mismatch. We can use the match expression to do so.

## The Match Control Flow Construct

When using match take care of all cases:
```python
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```
placeholder _ if no value required:
```python
use std::io;
fn main(){
    let dice_roll = 9;

    match dice_roll{
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other) # _ => reroll(), use placeholder, as value isn't needed.
    }
}
```
## Concise Control Flow with if let.
Option<T> is the way to handle exceptions in Rust. Equivalent in python would be try: except:

Useful Example:

```python
use std::collections::HashMap;

fn print_inventory(book_name: &str, book: Option<&u32>) {
    match book {
        Some(number) => println!("There are {} copies of {}", number, book_name),
        None         => println!("{} is out of stock :(", book_name)
    }
}

fn main() {
    let mut books: HashMap<&str, u32> = HashMap::new();
    books.insert("A silent voice", 5);

    // HashMap<&str, u32>::get() returns an Option<u32> (32-bit unsigned integer).
    let a_silent_voce = books.get("A silent voice");
    print_inventory("A silent voice", a_silent_voce);

    let spice_and_wolf = books.get("Spice and Wolf");
    print_inventory("Spice and Wolf", spice_and_wolf);
}
```
# Chapter 7 - Managing Growing Projects with Packages, Crates and Modules

## Dependencies
Dependencies in Cargo.toml
Example:
```python
[dependencies]
rand = "0.8.3" # same as ^0.8.3. Any version that is at least 0.8.3, but below 0.9.0

# Dependencies: External crates that the project depends on.
```
## Modules
Example: ./restaurant
Libraries can help programmers to find related functions or structs faster. Modules are defined in libraries.

code:
```python
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
## Seperate a module into multiple files
using
```python
<front_of_house.rs>
pub mod hosting;
```
rather than
```python
<front_of_house.rs>
pub mod hosting(){
  pub fn add_to_waitlist(){}
}

-> <src/front_of_house/hosting.rs>
pub fn add_to_waitlist(){
  #...
}

```
tells Rust to load the contents of the module from another file with the same name as the module.

# Chapter 8 - Common Collections

## Storing Lists of Values with Vectors

Instantiate a vector:
```python
let v: Vec<i32> = Vec::new(); # new vector
let v2 = vec![1,2,3];
v2.push(4);
```
Iterate over a mutable reference:
```python
let mut v = vec![100,32,57];
for i in &mut v{
  *i += 50; # add 50 to every element of the vector.
}
```

## A vector can store only one type. To store multiple types, we use an Enumerator.

Vector from enumerator:
```python
enum SpreadsheetCell{
  Int(i32),
  Float(f64),
  Text(String)
}

let row = vec![
  SpreadsheetCell::Int(3),
  SpreadsheetCell::Text(String::from("text")),
  SpreadsheetCell::Float(1.19)
];
```
-> Rust needs to know all the types that are in the vector at compile time.

## Hashmaps -> Very similar to a dictionary, makes dictionary obsolete.

Define a hashmap:
```python
use std::collections::HashMap;
let mut scores = HashMap.new();
scores.insert(String::from("Host"), 1);
scores.insert(String::from("Guest"), 1);
```

Construct a hashmap from two vectors, where vec1[n] is the key of the value vec2[n]:
```python
use std::collections::HashMap;
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10,50];

let mut scores: Hashmap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
```

## Accessing Values in a Hashmap
```python
let team_name = String::from("Blue");
score = scores.get(&team_name);
```
### Using a for-loop
```python
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 20);

for (key, value) in &scores{
  println!("{}: {}", key, value);
}
```

Overwrite by inserting a different value for the same key.
## Check if a Key has a Value or not, if not, associate a value with it.
```python
scores.entry(String::from("Yellow")).or_insert(60);
```

# Chapter 9 - Error Handling

## panic! ( Unrecoverable Error )
```python
fn main(){
  panic!("crash and burn");
}
```

Example where the vector library will panic:
```python
fn main(){
  let v = vec![1,2,3];
  v[100];
}
```
-> keyerror
## Result ( Recoverable Error )
```python
enum Result<T, E>{
  Ok(T),
  Err(E)
}
```

Find the type of File::open("file.txt"):
```python
let f: u32 = File::open("file.txt");
-> output: CompilerError -> "found enum "Result<File,std::io::Error>" "
```
Now use this information to handle a potential Error caused by File::open("filename"):
```python
use std::fs::File;

fn main(){
  let f = File::open("file.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => panic!("Problem opening file: {:?}", error)
  };
}
```
Now we can match different types of Errors with ErrorKind from the io library:
```python
use std::fs::File;
use std::io::ErrorKind;

fn main(){
  let f = File::open("file.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind(){
      ErrorKind::NotFound => match File::create("file.txt"){
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating file {:?}", e)
      },
      other_error => {
        panic!("Problem opening the file {:?}", other_error)
      }
    }
  };
}
```

# Chapter 9 - Error Handling
panic! and Result<T,E>
There are several different ways of handling Errors in Rust and some coding experience is necessary to understand, when to choose which implementation.

# Chapter 10 - Generic Types, Traits and Lifetimes
## Lifetime
```python
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```
if you want to declare a function where a result is being derived from an input, the input needs to stay in scope for as long as it is being worked with. For this you need to categorize it's lifetime.

An Example for this syntax is in Small_Practicals.


# Chapter 11 - Writing Automated Tests
Read through this chapter, learning by doing through the IO Project
(Chapter 12) - see .root/minigrep // 12.4 - Developing the Library's Functionality with Test-Driven Deployment e.g.

# Chapter 12 - An I/O Project: Building a Command Line Program
.root/minigrep Project.
 ## Environment variables in the powershell:

 ```python
 PS> $Env:CASE_INSENSITIVE=1; cargo run to poem.txt
 PS> Remove-Item Env:CASE_INSENSITIVE
 ```

 (suitable for example in minigrep project)


# Chapter 13

## Closures - The topic seems to not be covered well by the book, so additional source are used for a better understanding.

### A simple definition of what a closure is and does:
```text
According to The Book, closure is an anonymous function that can capture its environment. There are two things that have to be highlighted in this definition:

A closure can be imagined like a function;
Contrary to a function, it can capture its environment (capturing the environment means that in a closure you can use the variables defined outside the closure body but accessible in its scope).
more: https://zhauniarovich.com/post/2020/2020-12-closures-in-rust/
```
### Make use of closures so a function isn't called in cases where it's output value may not be used.
Example:
```python
    let expensive_result = |intensity|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity // return intensity
    };


    if intensity < 25{
        println!(
            "Today, do {} pushups",
            expensive_result(intensity)
        );
        println!(
            "Next, do {} situps",
            expensive_result(intensity)
        );
    } 
    else{
        if random_number == 3{
            # in this case we don't want to run the expensive calculation,
            # which is basically the reason we will implement a closure.
            println!("Take a break today! Remember to stay hydrated!");
        }
        else{
            println!(
                "Today, run for {} minutes",
                expensive_result(intensity)
            );
        }
    }
}
```

## To be done:
-
-
-

# To Mark: Currently on Chapter 13.1

the smaller coding examples from the Rust book, that i've done are in Small_Practicals/src/main.rs

## Latest Changes:

+ Continued to work with Closures / Chapter 13.1 and achieved a better understanding of the topic.