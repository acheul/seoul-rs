#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn test_isomorphsim1() {

    #[derive(PartialEq, Debug, Default, Isomorphism)]
    #[isomorphism(u8, list=[A, B(10)])]
    enum ABC {
      #[default] #[title("A")] A,
      #[into(1)] #[title("B")] B(i32),
      C,
    }

    let list = ABC::list();
    assert_eq!(list, vec![ABC::A, ABC::B(10)]);
  
    let a = ABC::A;
    let b = ABC::B(10);
    let c = ABC::C;

    // Into
    assert_eq!(Into::<u8>::into(&a), 0);
    assert_eq!(Into::<u8>::into(a), 0);
    assert_eq!(Into::<u8>::into(&b), 1);
    assert_eq!(b.title().as_str(), "B");
    assert_eq!(c.title().as_str(), "C");

    // From
    let x: ABC = 0u8.into();
    assert!(x==ABC::A);
    let x: ABC = 1u8.into();
    assert!(x==ABC::B(0));
    let x: ABC = 20u8.into();
    assert!(x==ABC::A);

    
    #[derive(PartialEq, Debug,Default, Isomorphism)]
    #[isomorphism(bool)]
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
    assert!(f==DEF::D);

    let list = DEF::list();
    assert_eq!(list, vec![DEF::D, DEF::E { e1: 0, e2: Vec::<usize>::new()}, DEF::F(false, 0)]);
  }

  #[test]
  fn test_isomorphsim2() {

    // use alias type to prevent derive macro's parsing error.
    type StaticStr = &'static str;

    #[derive(Debug, PartialEq, Default, Isomorphism)]
    #[isomorphism(into=[u8, i8, StaticStr])]
    pub enum CD {
      #[default] #[into([0, 1, "c"])] C,
      #[into([0, -1, "d"])] D,
    }

    let c = CD::C;
    let d = CD::D;

    assert_eq!(Into::<u8>::into(&c), 0);
    assert_eq!(Into::<i8>::into(&c), 1);
    assert_eq!(Into::<u8>::into(&d), 0);
    assert_eq!(Into::<i8>::into(&d), -1);

    assert_eq!(Into::<CD>::into(1i8), CD::C);
    assert_eq!(Into::<CD>::into(-1i8), CD::D);

    let c: &str = c.into();
    assert_eq!(c, "c");
  }
}