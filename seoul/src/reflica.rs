pub use seoul_derive::Reflica;

/**
# Trait Reflica

* Declare a borrowed fields' data type ("reflica") from an original struct/enum data type, and implement Into trait to the reflica.

* concept of reflica:
  * `struct AB { a: u8, b: String }`
    * declare `struct RefAB<'a> { a: &'a u8, b: &'a String }`
    * implement `Into<RefAB<'a>> for &'a AB`  
 * `enum AB { A, B { a: u8, b: String } }`
    * declare `enum RefAB<'a> { A, B { a: &'a u8, b: &'a String } }`
    * implement `Into<RefAB<'a>> for &'a AB`


## Example
```rust
# use seoul::Reflica;

// struct
#[derive(Reflica)]
// attribute for derive implementation for the reflica 
#[reflica(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AB<I> {
  a: u8,
  b: I
}

// RefAB delcared
let _: RefAB<String> = RefAB { a: &8, b: &String::from("ab") };

// check Into<RefAB> for &AB
let x = AB { a: 0, b: String::from("x")};
let _: RefAB<String> = (&x).into();

// check derive `Ord`
let a: RefAB<u8> = RefAB { a: &2, b: &10 };
let b: RefAB<u8> = RefAB { a: &2, b: &11 };
let c: RefAB<u8> = RefAB { a: &3, b: &10 };
let x = vec![a, b, c];
let mut y = x.clone();
y.sort();
assert_eq!(x, y);


// enum, use prefix other than basic `Ref`
#[derive(Reflica)]
// `prefix` attribute's string value will be used for reflica's prefix, other than the basic prefix `Ref`
#[reflica(Clone, Copy, Debug, prefix="Ref2")]
enum ABC<I> where I: Clone {
  A,
  B { a: u8, b: I },
  C(u8)
}

// Ref2AB delcared
let _: Ref2ABC<u8> = Ref2ABC::A;
let _: Ref2ABC<String> = Ref2ABC::B { a: &8, b: &String::from("ab") };

// check Into<Ref2AB>
let x = ABC::B { a: 0, b: String::from("x")};
let _: Ref2ABC<String> = (&x).into();

let x = ABC::<u8>::C(0);
let _: Ref2ABC<u8> = (&x).into();
```
 */
pub trait Reflica: Sized {

}