use seoul::Isomorphism;

#[test]
fn test_isomorphism_variants1() {

    #[derive(Debug, Clone, PartialEq, Isomorphism)]
    #[isomorphism(from_variant, name_variant)]
    enum ABC {
        A(u32),
        B { _a: String, _b: Vec<i32> },
        C,
        D(u32, i32),
    }

    let x: ABC = 10u32.into();
    assert_eq!(x, ABC::A(10u32));
    assert_eq!(x.name(), "A");

    let x: ABC = ("string".to_string(), vec![10i32]).into();
    assert_eq!(x, ABC::B { _a: "string".to_string(), _b: vec![10i32] });
    assert_eq!(x.name(), "B");

    let x: ABC = ().into();
    assert_eq!(x, ABC::C);

    let x: ABC = (10u32, 11i32).into();
    assert_eq!(x, ABC::D(10u32, 11i32));
}

#[test]
fn test_isomorphism_variants2() {

    #[derive(Debug, Clone, PartialEq, Isomorphism)]
    #[isomorphism(from_variant, name_variant = "title")]
    enum ABC<T> {
        #[from_variant(skip)]
        _A(u32),
        B(u32),
        #[name("C2")]
        C(u32),
        #[from_variant(skip)]
        _D(T),
    }

    let x: ABC<()> = 10u32.into();
    assert_eq!(x, ABC::B(10u32));
    
    assert_eq!(ABC::<()>::C(10u32).title(), "C2");
}


#[derive(Debug, Clone, PartialEq, Isomorphism)]
#[isomorphism(name_variant, into = u8, restore_panic = "NOT MATCHED")]
enum XYZ<T> {
    #[into(10u8)]
    X,
    #[into(20u8)]
    Y { a: Vec<T> },
    #[from_variant]
    Z(u8, T),
}

#[test]
fn test_isomorphism_variants3() {

    assert_eq!(Into::<u8>::into(XYZ::<()>::X), 10u8);
    assert_eq!(Into::<u8>::into(&XYZ::<()>::Z(10u8, ())), 0u8);

    assert_eq!(Into::<XYZ<()>>::into(10u8), XYZ::X);
    assert_eq!(Into::<XYZ<()>>::into(&20u8), XYZ::Y { a: Vec::new() });

    assert_eq!(XYZ::<()>::X.name(), "X");
    assert_eq!(Into::<XYZ<()>>::into((10u8, ())), XYZ::Z(10u8, ()));
}

#[test]
#[should_panic]
fn test_isomorphism_variants3_2() {
    let _ = Into::<XYZ<()>>::into(100u8);
}


#[test]
fn test_isomorphism_variants4() {

    #[derive(Debug, Clone, PartialEq, Isomorphism)]
    #[isomorphism(into = u8, default_into = 5u8, default_restore = ABC::A)]
    enum ABC {
        #[into(10)]
        A,
        #[into(20)]
        B,
        #[into(30)]
        C,
        D,
    }

    assert_eq!(Into::<u8>::into(ABC::A), 10u8);
    assert_eq!(Into::<u8>::into(ABC::D), 5u8);

    assert_eq!(Into::<ABC>::into(&20), ABC::B);
    assert_eq!(Into::<ABC>::into(0), ABC::A);
}

#[test]
fn test_isomorphism_variants5() {

    type Str = &'static str;

    #[derive(Default, Debug, Clone, PartialEq, Isomorphism)]
    #[isomorphism(into = Str, skip_ref_restore)]
    enum ABC {
        #[into("a")]
        A,
        #[into("b")]
        B,
        #[into("c")]
        C,
        #[default]
        D,
    }

    assert_eq!(Into::<&str>::into(ABC::A), "a");
    assert_eq!(Into::<&str>::into(ABC::D), "");

    assert_eq!(Into::<ABC>::into("b"), ABC::B);
    assert_eq!(Into::<ABC>::into("x"), ABC::D);
}