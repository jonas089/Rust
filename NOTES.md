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

let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);

s1 is now invalid and s2 becomes the only reference to the data on the heap.

-----

let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);

types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y.

-----
