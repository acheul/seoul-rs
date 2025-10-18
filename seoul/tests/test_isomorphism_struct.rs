use seoul::Isomorphism;

#[test]
fn test_isomorphism_fields1() {

    #[derive(Debug, Clone, PartialEq, Isomorphism)]
    #[isomorphism(into_field)]
    struct ABC {
        a: i32,
        b: Vec<i32>,
        c: String,
        #[into_field(skip)]
        d: u32,
        e: i32,
    }

    let mut abc = ABC {
        a: 10i32,
        b: vec![10i32, 20, 30],
        c: "ABC".to_string(),
        d: 20u32, e: 30i32,
    };

    assert_eq!(Into::<&i32>::into(&abc), &10i32);
    assert_eq!(Into::<&Vec<i32>>::into(&abc), &[10, 20, 30]);
    assert_eq!(Into::<&String>::into(&abc), "ABC");
    
    // let x = Into::<&u32>::into(&abc); // Not available

    let mut_b: &mut Vec<i32> = (&mut abc).into();
    mut_b.push(40);
    assert_eq!(&abc.b, &[10, 20, 30, 40]);

    assert_eq!(Into::<String>::into(abc), "ABC".to_string());
}


#[test]
fn test_isomorphism_fields2() {

    #[derive(Debug, PartialEq, Isomorphism)]
    struct ABC<'a, T, U> {
        #[into_field]
        a: Vec<T>,
        #[into_field]
        b: &'a String,
        #[into_field]
        c: &'a mut Vec<U>,
        d: u32,
    }

    let mut abc = ABC::<usize, String> {
        a: vec![1usize, 2],
        b: &String::from("B"),
        c: &mut vec!["C".to_string()],
        d: 10u32
    };

    assert_eq!(Into::<&Vec<usize>>::into(&abc), &[1, 2]);
    assert_eq!(Into::<&&String>::into(&abc), &"B");
    assert_eq!(Into::<&&mut Vec<String>>::into(&abc), &&mut vec!["C".to_string()]);

    // =let x = Into::<u32>::into(&abc); // Not available

    let mut_c = Into::<&mut &mut Vec<String>>::into(&mut abc);
    mut_c.pop();
    assert!(abc.c.is_empty());

    let c = Into::<&mut Vec<String>>::into(abc);
    c.push("C".to_string());
    assert_eq!(c.as_slice(), &["C".to_string()]);
}


#[test]
fn test_isomorphism_fields3() {

    #[derive(Debug, Clone, PartialEq, Isomorphism)]
    #[isomorphism(into_field, intofrom_tuple)]
    struct ABC<T> {
        a: i32,
        b: String,
        c: Vec<T>,
    }

    let mut abc: ABC<String> = ABC {
        a: 10i32,
        b: "ABC".to_string(),
        c: vec!["A".to_string()],
    };

    assert_eq!(Into::<&i32>::into(&abc), &10i32);
    
    let x: (&mut i32, &mut String, &mut Vec<String>) = (&mut abc).into();
    x.2.push("B".to_string());

    let x: (&i32, &String, &Vec<String>) = (&abc).into();
    assert_eq!(x.2, &["A".to_string(), "B".to_string()]);

    let (a, b, mut c): (i32, String, Vec<String>) = abc.into();
    c.push("C".to_string());

    let abc: ABC<String> = (a, b, c).into();
    assert!(abc.c.len() == 3);
}