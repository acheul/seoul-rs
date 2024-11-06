use seoul::*;

#[test]
fn test_intowrap() {

  #[derive(Debug, Clone, PartialEq, IntoWrap)]
  enum ABCD {
    A,
    B(i32),
    C { a: i32 },
    D(String)
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
    A((X, X)),
    B(Vec<Y>)
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
}