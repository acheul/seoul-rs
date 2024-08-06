use seoul::*;

#[test]
fn test_reflica1() {

  // struct
  #[derive(Reflica)]
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


  // unnamed fields
  #[derive(Reflica)]
  struct CD(u8, u8);

  let _: RefCD = RefCD(&0, &10);
}


#[test]
fn test_reflica2() {

  // enum, use prefix other than basic `Ref`
  #[derive(Reflica)]
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
}