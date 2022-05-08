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





# To Mark: Currently on Chapter 6.1
most coding examples i've done are in Small_Practicals/src/main.rs
## Latest Changes:
learned to use enums together with structs and worked through some additional tutorial.
