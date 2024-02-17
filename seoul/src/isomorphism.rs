pub use seoul_derive::Isomorphism;

/// Trait Isomorphism
/// 
/// Using derive macro, you can implement trait Into<T> for Self and &Self, and From<T> for Self too.
/// 
pub trait Isomorphism: Sized {

  fn title(&self) -> String;

  fn list() -> Vec<Self>;
}