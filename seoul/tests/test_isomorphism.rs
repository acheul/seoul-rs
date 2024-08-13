use seoul::*;

#[test]
fn test_isomorphism1() {

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


  // `has_default` enables `From<T>` and `From<&T>` trait for `Self`
  #[derive(Default, PartialEq, Debug, Isomorphism)]
  #[isomorphism(bool, has_default)]
  enum DEF {
    #[default] D,
    #[into(true)]
    E {
      e1: i32, e2: Vec<usize>
    },
    F(bool, usize)
  }

  let d = DEF::D;
  let e = DEF::E { e1: 10, e2: vec![1, 2, 3 ] };
  let f = DEF::F(true, 100);
  
  // Into<bool>
  assert!(!Into::<bool>::into(&d));
  assert!(Into::<bool>::into(&e));
  assert!(!Into::<bool>::into(f));

  // From<bool>
  let t: DEF = true.into();
  let f: DEF = false.into();
  assert!(t==DEF::E{e1:0, e2:Vec::<usize>::new()});
  assert!(f==DEF::default());

  let list = DEF::list();
  assert_eq!(list, vec![DEF::D, DEF::E { e1: 0, e2: Vec::<usize>::new()}, DEF::F(false, 0)]);
}

#[test]
fn test_isomorphism2() {

  // use alias type to prevent derive macro's parsing error when the type's raw name has white space;
  type StaticStr = &'static str;

  #[derive(Debug, PartialEq, Isomorphism)]
  #[isomorphism(into=[u8, i8, StaticStr])]
  pub enum CD {
    #[into([0, 1, "c"])] C,
    #[into([0, -1, "d"])] D,
  }

  let c = CD::C;
  let d = CD::D;

  assert_eq!(Into::<u8>::into(&c), 0);
  assert_eq!(Into::<i8>::into(&c), 1);
  assert_eq!(Into::<u8>::into(&d), 0);
  assert_eq!(Into::<i8>::into(&d), -1);

  let c: &str = c.into();
  assert_eq!(c, "c");
}

/// parsing generics
#[test]
fn test_isomorphism3() {

  #[derive(Debug, PartialEq, Isomorphism)]
  #[isomorphism(u8)]
  pub enum AB<X: Default> {
    #[into(0)] A(X),
    #[into(1)] B(String)
  }

  let x: AB<String> = AB::A("A".to_string());
  assert_eq!(Into::<u8>::into(&x), 0);
  assert_eq!(x.title(), "A");

  let x: AB<i32> = AB::B("B".to_string());
  assert_eq!(Into::<u8>::into(&x), 1);
  assert_eq!(x.title(), "B");
}


/// on Struct type
#[test]
fn test_isomorphism4() {

  #[derive(Debug, Default, PartialEq, Isomorphism)]
  pub struct AB { _a: String, _b: i32 }

  let x = AB::default();
  assert!(x.title().is_empty());
  assert!(AB::list().is_empty());
}