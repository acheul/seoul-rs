pub use seoul_derive::Isomorphism;

/// # Trait Isomorphism
/// 
/// * Using derive macro, you can implement
///   * trait `Isomorphism`
///     - `fn title(&self) -> &str;`
///     - `fn list() -> Vec<Self>;`
///   * trait `Into<T>` for `&Self` and `Self`
///   * trait `From<T>` and `From<&T>` for `Self`, when the type implements trait `Default`
/// 
/// * Works only on `Enum` type
/// 
/// # derive Syntax
/// * When **title** is not given at variant level, **the variant's name (Ident)** will be used as titile.
/// * When **list** is not given at the top level attribute, list of each variant's default format will be returned.
/// * When **into** value is not given at variant level, the into type(T)'s default value will be used in `<Into<T>>` trait.
/// * When the type implements `Default` and **has_default** is given at top level attribute, the `From<T>` and `From<&T>` will be implemented for each given into types(T). 
/// 
/// # Ex
/// ```
/// # use seoul::Isomorphism;
/// 
/// #[derive(PartialEq, Debug, Isomorphism)]
/// #[isomorphism(u8, list=[A, B(10)])]
/// enum ABC {
///   #[into(0)] #[title("A")] A,
///   #[into(1)] B(i32),
///   C,
/// }
///
/// // `list()`
/// let list = ABC::list();
/// assert_eq!(list, vec![ABC::A, ABC::B(10)]);
/// 
/// let a: ABC = ABC::A;
/// let b = ABC::B(10);
/// let c = ABC::C;
/// 
/// // `title()`
/// assert_eq!(a.title(), "A");
/// assert_eq!(b.title(), "B");
/// assert_eq!(c.title(), "C");
/// 
/// // `Into<T>` for `&Self` and `Self`
/// assert_eq!(Into::<u8>::into(&a), 0);
/// assert_eq!(Into::<u8>::into(a), 0);
/// assert_eq!(Into::<u8>::into(&b), 1);
/// assert_eq!(Into::<u8>::into(c), 0);
/// 
/// 
/// #[derive(Default, Debug, Isomorphism, PartialEq)]
/// #[isomorphism(into=[u8, i8], has_default)]
/// pub enum CD {
///   #[default] #[into([0, 1])] C,
///   #[into([0, -1])] D,
/// }
/// 
/// // list
/// assert_eq!(CD::list(), vec![CD::C, CD::D]);
/// 
/// // Into
/// assert_eq!(Into::<u8>::into(CD::C), 0);
/// assert_eq!(Into::<i8>::into(CD::C), 1);
/// assert_eq!(Into::<u8>::into(CD::D), 0);
/// assert_eq!(Into::<i8>::into(CD::D), -1);
/// 
/// // From
/// assert_eq!(Into::<CD>::into(1i8), CD::C);
/// assert_eq!(Into::<CD>::into(-1i8), CD::D);
/// // fallback to default value of `CD`
/// assert_eq!(Into::<CD>::into(-0i8), CD::C);
/// ```
pub trait Isomorphism: Sized {

  fn title(&self) -> &str;

  fn list() -> Vec<Self>;
}