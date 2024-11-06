pub use seoul_derive::IntoWrap;

/**
# Trait IntoWrap
* Implement `From` traits for each variants of an **enum** type data,
  where transforming field's data into the enum data
  when the variant has one unnamed field.

* Thus each variants should not have any redundant type or generic to avoid any conflicting implementations.

* `IntoWrap` can be though as a not-tuple single variable version of `Tuplike` for enum's wrapping variants.

## Example
```rust
use seoul::IntoWrap;

#[derive(Debug, Clone, PartialEq, IntoWrap)]
enum ABCD {
  A, // skip
  B(i32), // impl From<i32> for ABCD
  C { a: i32 }, // skip
  D(String) // impl From<String> for ABCD
}

let _ = ABCD::A;
let _ = ABCD::C { a: 10 };

let b: i32 = 10;
let b: ABCD = b.into();

let d = "string".to_string();
let d: ABCD = d.into();

assert_eq!(b, ABCD::B(10));
assert_eq!(d, ABCD::D("string".to_string()));


#[derive(Debug, Clone, PartialEq, IntoWrap)]
enum AB<X: Clone, Y> where Y: Clone {
  A((X, X)), // impl<X: Clone, Y> From<X, X> for AB where Y: Clone
  B(Vec<Y>) // impl<X: Clone, Y> From<Vec<X>> for AB where Y: Clone
}

let a: (String, String) = ("x".to_string(), "x".to_string());
let a: AB<String, i32> = a.into();

let b: Vec<i32> = vec![0, 1, 2];
let b: AB<String, i32> = b.into();

assert_eq!(a, AB::<String, i32>::A(("x".to_string(), "x".to_string())));
assert_eq!(b, AB::<String, i32>::B(vec![0, 1, 2]));


// This case won't work due to conflicting implementations.
/*#[derive(Debug, Clone, PartialEq, IntoWrap)]
enum AB<X: Clone, Y> where Y: Clone {
  A(X),
  B(Vec<Y>)
}*/


// for struct
#[derive(Debug, Clone, PartialEq, IntoWrap)]
struct ExStruct(String);

let _x: ExStruct = String::from("ab").into();

#[derive(Debug, Clone, PartialEq, IntoWrap)]
struct ExStruct2<X: Clone> {
  a: X
}

let _x: ExStruct2<String> = String::from("ab").into();
```
 */
pub trait IntoWrap: Sized { }