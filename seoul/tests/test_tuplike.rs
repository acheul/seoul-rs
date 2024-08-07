use seoul::*;

#[test]
fn test_tuplike1() {

  #[derive(Debug, Clone, PartialEq, Tuplike)]
  struct AB {
    a: u8, b: String
  }

  let tuple_: (u8, String) = (0, "string".to_string());
  let ab_: AB = AB { a: 0, b: "string".to_string() };

  //let _: <AB as Tuplike>::Tuple = ab_.clone().into();

  let ab_into: (u8, String) = ab_.clone().into();
  let tuple_into: AB = tuple_.clone().into();

  assert_eq!(&ab_into, &tuple_);
  assert_eq!(&tuple_into, &ab_);

  let _ab_ref_into: (&u8, &String) = (&ab_).into();
}

#[test]
fn test_tuplike2() {

  #[derive(Debug, Clone, PartialEq, Tuplike)]
  struct AB(u8, String);

  let tuple_: (u8, String) = (0, "string".to_string());
  let ab_: AB = AB(0, "string".to_string());

  //let _: <AB as Tuplike>::Tuple = ab_.clone().into();

  let ab_into: (u8, String) = ab_.clone().into();
  let tuple_into: AB = tuple_.clone().into();

  assert_eq!(&ab_into, &tuple_);
  assert_eq!(&tuple_into, &ab_);

  let _ab_ref_into: (&u8, &String) = (&ab_).into();
}


#[test]
fn test_tuplike3() {

  #[derive(Debug, Clone, PartialEq, Tuplike)]
  struct AB<X: Clone> {
    a: u8, b: X
  }

  let tuple_: (u8, String) = (0, "string".to_string());
  let ab_: AB<String> = AB { a: 0, b: "string".to_string() };

  //let _: <AB<String> as Tuplike>::Tuple = ab_.clone().into();

  let ab_into: (u8, String) = ab_.clone().into();
  let tuple_into: AB<String> = tuple_.clone().into();

  assert_eq!(&ab_into, &tuple_);
  assert_eq!(&tuple_into, &ab_);

  let _ab_ref_into: (&u8, &String) = (&ab_).into();
}

#[test]
fn test_tuplike4() {
  #[derive(Debug, Clone, PartialEq, Tuplike)]
  struct AB<X: Clone, Y> where Y: Clone {
    a: X, b: Y
  }

  let tuple_: (u8, String) = (0, "string".to_string());
  let ab_: AB<u8, String> = AB { a: 0, b: "string".to_string() };

  //let _: <AB<u8, String> as Tuplike>::Tuple = ab_.clone().into();

  let ab_into: (u8, String) = ab_.clone().into();
  let tuple_into: AB<u8, String> = tuple_.clone().into();

  assert_eq!(&ab_into, &tuple_);
  assert_eq!(&tuple_into, &ab_);

  let _ab_ref_into: (&u8, &String) = (&ab_).into();
}

#[test]
fn test_tuplike5() {
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
}