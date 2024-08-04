pub use seoul_derive::Tuplike;

/// # Trait Tuplike
/// 
/// * Transform a **struct** data into **tuple** format
///   * `AB { a: 0, b: 10 }` <=> `(0, 10)`
/// 
/// * Using derive macro, you can implement
///   * trait `From<T>` for `Self`
///   * trait `Into<T>` for `Self`
///   * trait `Into<R>` for `Self`
///   * Hereby, `T` is a tuple format of the struct's fields,
///   * and the `R` is a referenced tuple format of them.
/// 
/// * The trait `Tuplike` has an associated type `Tuple`.
///   With the derive macro, the tuple format will be assigned for it.
/// 
/// # Ex
/// ```
/// # use seoul::Tuplike;
/// 
/// #[derive(Debug, Clone, PartialEq, Tuplike)]
///  struct AB {
///    a: u8, b: String
///  }
///
///  let tuple_: (u8, String) = (0, "string".to_string());
///  let ab_: AB = AB { a: 0, b: "string".to_string() };
/// 
///  let _: <AB as Tuplike>::Tuple = ab_.clone().into();
///
///  let ab_into: (u8, String) = ab_.clone().into();
///  let tuple_into: AB = tuple_.clone().into();
///
///  assert_eq!(&ab_into, &tuple_);
///  assert_eq!(&tuple_into, &ab_);
///
///  let _ab_ref_into: (&u8, &String) = (&ab_).into();
/// ```
/// 
pub trait Tuplike: Sized {

  type Tuple;
}