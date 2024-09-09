# Seoul-Rs

[![Crates.io](https://img.shields.io/crates/v/seoul)](https://crates.io/crates/seoul)
[![docs.rs](https://img.shields.io/docsrs/seoul?color=blue&label=docs.rs)](https://docs.rs/seoul)

* derive trait **Isomorphism** for **Enum** data
* derive trait **Tuplike** for **Struct** and **Enum** data
* derive trait **Reflica** for **Struct** and **Enum** data
* derive trait **IntoWrap** for **Enum** data


# Trait Isomorphism
* for **enum** data type. Convenient transformation of enum values with derive macro.

* Basic methods:
    * `fn title(&self) -> String;`
    * `fn list() -> Vec<Self>;`

* The derive macro also implements
    * `Into<T>` for `Self` and `&Self`
    * `From<T>` and `From<&T>` for `Self` (when `Default` is implemented and "has_default" syntax is given)

## derive syntax and fallback
* When **title** is not given at variant level, **the variant's name (Ident)** will be used as titile.
* When **list** is not given at the top level attribute, list of each variant's default format will be returned.
* When **into** value is not given at variant level, the into type(T)'s default value will be used in `<Into<T>>` trait.
* When the type implements `Default` and **has_default** is given at top level attribute, the `From<T>` and `From<&T>` will be implemented for each given into types(T). 

## Examples
```rust
use seoul::Isomorphism;

#[derive(PartialEq, Debug, Isomorphism)]
#[isomorphism(u8, list=[A, B(10)])]
enum ABC {
  #[into(0)] #[title("A")] A,
  #[into(1)] B(i32),
  C,
}

// `list()`
let list = ABC::list();
assert_eq!(list, vec![ABC::A, ABC::B(10)]);

let a: ABC = ABC::A;
let b = ABC::B(10);
let c = ABC::C;

// `title()`
assert_eq!(a.title(), "A");
assert_eq!(b.title(), "B");
assert_eq!(c.title(), "C");

// `Into<T>` for `&Self` and `Self`
assert_eq!(Into::<u8>::into(&a), 0);
assert_eq!(Into::<u8>::into(a), 0);
assert_eq!(Into::<u8>::into(&b), 1);
assert_eq!(Into::<u8>::into(c), 0);
```

```rust
#[derive(Default, Debug, Isomorphism, PartialEq)]
#[isomorphism(into=[u8, i8], has_default)]
pub enum CD {
  #[default] #[into([0, 1])] C,
  #[into([0, -1])] D,
}

// list
assert_eq!(CD::list(), vec![CD::C, CD::D]);

// Into
assert_eq!(Into::<u8>::into(CD::C), 0);
assert_eq!(Into::<i8>::into(CD::C), 1);
assert_eq!(Into::<u8>::into(CD::D), 0);
assert_eq!(Into::<i8>::into(CD::D), -1);

// From
assert_eq!(Into::<CD>::into(1i8), CD::C);
assert_eq!(Into::<CD>::into(-1i8), CD::D);
// fallback to default value of `CD`
assert_eq!(Into::<CD>::into(-0i8), CD::C);
```

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

# Trait Reflica
* Declare a borrowed fields' data type ("reflica") from an original struct/enum data type, and implement Into trait to the reflica.

* concept of reflica:
  * `struct AB { a: u8, b: String }` ->
    * declare `struct RefAB<'a> { a: &'a u8, b: &'a String }`
    * implement `Into<RefAB<'a>> for &'a AB`  
  * `enum AB { A, B { a: u8, b: String } }` ->
    * declare `enum RefAB<'a> { A, B { a: &'a u8, b: &'a String } }`
    * implement `Into<RefAB<'a>> for &'a AB`


## Example
```rust
use seoul::Reflica;

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
```

## Dev Log
```yaml
- ver.0.2.0
  - `From<T>` implemented only with `has_default` attribute syntax when `Default` is implemented.
- ver 0.2.1~2
  - correct some typos

- ver 0.3.0
  - Add `Tuplike`.
- ver 0.3.1
  - Add an associated type `Tuple` to the trait `Tuplike`

- ver 0.3.2
  - Add `Reflica`

- ver 0.3.3
  - On `Tuplike`:
    - delete associated type `Tuple` from the trait.
    - derive macro now works on the enum type too: but only `From<T>` trait will be implemented for each variant.
  - Add `IntoWrap`

- ver 0.3.5
  - On `Isomorphism`:
    - Reinforced derive macro's generic parsing ability.
    - For struct type, the derive macro would only implement the Isormophism trait with each methods of it returning default values.

- ver 0.3.6
  - crate `seoul-derive`:
    - a bit revision of error comments;
  - On `Reflica`: copy visibility of a type and fields; (pub, pub(crate), etc.)
```