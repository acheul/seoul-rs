pub use seoul_derive::Tuplike;

/**
# Trait Tuplike
* for **struct** data type. Transform a **struct** data into **tuple** format
  * `AB { a: 0, b: 10 }` <=> `(0, 10)`

* Using derive macro, you can implement
  * trait `From<T>` for `Self`
  * trait `Into<T>` for `Self`
  * trait `Into<R>` for `&Self`
  * Hereby, `T` is a tuple format of the struct's fields,
  * and the `R` is a referenced tuple format of them.

* for the **enum** data type, Only `From<T>` trait will be implemted for each variant.

## Example
```rust
use seoul::Tuplike;

#[derive(Debug, Clone, PartialEq, Tuplike)]
struct AB {
  a: u8, b: String
}

let tuple_: (u8, String) = (0, "string".to_string());
let ab_: AB = AB { a: 0, b: "string".to_string() };

let ab_into: (u8, String) = ab_.clone().into();
let tuple_into: AB = tuple_.clone().into();

assert_eq!(&ab_into, &tuple_);
assert_eq!(&tuple_into, &ab_);

let _ab_ref_into: (&u8, &String) = (&ab_).into();


// for enum, just `From<T>` will be implemented for each variant.
#[derive(Debug, Clone, PartialEq, Tuplike)]
enum ABC {
  A,
  B(String),
  C { a: i32, b: String }
}

let _ = ABC::A;

let b1: (String,) = ("string".to_string(),);
let b2: ABC = ABC::B("string".to_string());

let b1_: ABC = b1.clone().into();

assert_eq!(&b1_, &b2);

let c1: (i32, String) = (10, "string".to_string());
let c2: ABC = ABC::C { a: 10, b: "string".to_string() };
let c1_: ABC = c1.clone().into();

assert_eq!(&c1_, &c2);
```
 */
pub trait Tuplike: Sized { }