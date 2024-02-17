# Seoul-Rs

[![Crates.io](https://img.shields.io/crates/v/seoul)](https://crates.io/crates/seoul)
[![docs.rs](https://img.shields.io/docsrs/seoul?color=blue&label=docs.rs)](https://docs.rs/seoul)


## Trait Isomorphism
  * To handle **enum** data type. Convenient transformation of enum values with derive macro.
  * Basic methods:
    * `fn title(&self) -> String;`
    * `fn list() -> Vec<Self>;`
  * Using derive macro, also are implemented
    * `Into<T> for Self`
    * `Into<T> for &Self`
    * `From<T> for Self`

  ### Examples
```rust
#[derive(Default, Isomorphism)]
#[isomorphism(u8, list=[A, B])]
pub enum ABC {
  #[default] #[title("a")] A,
  #[into(10)] #[title("b")] B,
  #[into(100)] C
}

// Into
assert_eq!(Into::<u8>::into(ABC::A), 0);
assert_eq!(Into::<u8>::into(&ABC::B), 10);
// From
assert_eq!(Into::<ABC>::into(0u8), ABC::A);
assert_eq!(Into::<ABC>::into(100u8), ABC::C);
// List
assert_eq!(ABC::list(), vec![ABC::A, ABC::B]);
// Title
assert_eq!(ABC::A.title(), "A");
assert_eq!(ABC::C.title(), "C");


#[derive(Default, Isomorphism)]
#[isomorphism(into=[u8, i8])]
pub enum CD {
  #[default] #[into([0, 1])] C,
  #[into([0, -1])] D,
}

// Into
assert_eq!(Into::<u8>::into(CD::C), 0);
assert_eq!(Into::<i8>::into(CD::C), 1);
assert_eq!(Into::<u8>::into(CD::D), 0);
assert_eq!(Into::<i8>::into(CD::D), -1);
// From
assert_eq!(Into::<CD>::into(1i8), CD::C);
assert_eq!(Into::<CD>::into(-1i8), CD::D);
```