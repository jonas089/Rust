# Cargo Commands

cargo new -> create cargo tree
cargo build  -> compile cargo project
cargo run -> compile and run cargo project
cargo check -> check if code can be compiled ( faster than compiling )


# Stack and Heap

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

# Copies of Variables

<code>
  let s1 = String::from("hello");
  let s2 = s1;

  println!("{}, world!", s1);
</code>

s1 is now invalid and s2 becomes the only reference to the data on the heap.

-----

<code>
  let x = 5;
  let y = x;

  println!("x = {}, y = {}", x, y);
</code>

types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y.

-----

## "Copy"

All the integer types, such as u32.
The Boolean type, bool, with values true and false.
All the floating point types, such as f64.
The character type, char.
Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

# Returning a tuple ( multiple values )

<code>
  fn main() {
      let s1 = String::from("hello");

      let (s2, len) = calculate_length(s1);

      println!("The length of '{}' is {}.", s2, len);
  }

  fn calculate_length(s: String) -> (String, usize) {
      let length = s.len(); // len() returns the length of a String

      (s, length)
  }
</code>

## References and Borrowing

Reference to an object as a parameter instead of taking ownership of the value of the object. / s1 "survives" the entire scope.

<code>
  fn main() {
      let s1 = String::from("hello");

      let len = calculate_length(&s1); //

      println!("The length of '{}' is {}.", s1, len);
  }

  fn calculate_length(s: &String) -> usize {
      s.len()
  }
</code>
