# Some Notes for me to look into occasionally


s.as_bytes() // convert string to bytes

# In C and C++
. if you're calling a method on the object directly
-> if you're calling a method on a pointer to the object
and need to dereference the pointer first. In other words,
if object is a pointer, object->something() is similar to
(*object).something().

Rust doesn't have an equivalent to the -> operator, instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust that has this behavior.

This is the same:

```python
p1.distance(&p2);
(&p1).distance(&p2);
```
