# Seoul-Rs

## A Procedural Macros Toolkit for Struct and Enum Types' Transformation

[![Crates.io](https://img.shields.io/crates/v/seoul)](https://crates.io/crates/seoul)
[![docs.rs](https://img.shields.io/docsrs/seoul?color=blue&label=docs.rs)](https://docs.rs/seoul)

✈️ _V1.0.0 has major updates from former versions. See [Version Log](VERLOG.md) for changes._

### TOC

- [Isomorphism for Struct](#isomorphism-for-struct)
- [Isomorphism for Enum](#isomorphism-for-enum)
- [Reflica](#trait-reflica)

## Trait Isomorphism

Implement bunch of `Into` and `From` traits for isomorphic transformation.

## Isomorphism for `Struct`

### _Abilities_

- **into_field**: _Self ➡️ Each Field_
  - impl `Into<Field>` for `Self`
  - impl `Into<&Field>` for `&Self`
  - impl `Into<&mut Field>` for `&mut Self`
- **intofrom_tuple**: _Self ↔️ Tuple of Fields_
  - impl `From<(Field1, Field2, ..)>` for `Self`
  - impl `Into<(Field1, Field2, ..)>` for `Self`
  - impl `Into<(&Field1, &Field2, ..)>` for `&Self`
  - impl `Into<(&mut Field1, &mut Field2, ..)>` for `&mut Self`

### _Attributes_

- _Container Attributes_
  - `#[isomorphism(into_field)]`
    - Invoke **into_field**'s implements for each fields.
    - Fields with `#[into_field(skip)]` attribute or redundant types already captured in preceding fields will not be invoked.

  - `#[isomorphism(intofrom_tuple)]`
    - Invoke **intofrom_tuple**'s implements.

- _Field Attributes_
  - `#[into_field]` / `#[into_field(skip)]`
    - Fields with `#[into_field]` attribute still invoke **into_field** regardless of the container attribute.
    - However ones with `#[into_field(skip)]` do not invoke **into_field** implements for themselves even though the container attribute has `into_field`.

### _Examples_

```rust
use seoul::Isomorphism;

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
```

## Isomorphism for `Enum`

### _Abilities_

- **from_variant**: _Each Variant ➡️ Self_
  - impl `From<Varaint>` for `Self`
- **name_variant**
  - Add a method with signature `pub fn name(&self) -> &str` to the type's implement scope.
  - The method matches each variant into the variant's own name literal, while it can be specified differently by a variant attribute `name`.
- **convert**: _Self ↔️ Other Type_
  - Convert each variant to other type's matching value, and vice versa.
  - impl `Into<OtherType>` for `Self`
  - impl `Into<&OtherType>` for `Self`
  - impl `From<OtherType>` for `Self`
  - impl `From<&OtherType>` for `Self`

### _Attributes_

- _Container Attributes_
  - `#[isomorphism(from_variant)]`
    - Invoke **from_variant**'s implements for each variants.
      - For a variant with one field: `impl From<Field> for Self`
      - One with multiple fields: `impl From<(Field1, Field2, ..)> for Self`
      - One with no fields: `impl From<()> for Self`
    - Fields with `#[from_variant(skip)]` attribute or redundant types already captured in preceding variants will not be invoked.

  - `#[isomorphism(name_variant)]` / `#[isomorphism(name_variant = "specific_method_name")]`
    - Invoke **name_variant**.
    - While the default name of the method is `name`, it can be changed by passing a specific name in the attribute.

  - For **convert**:
    - `#[isomorphism(into = u8)]`
      - Invoke **convert**. Specify the type for the convertion.

    - `#[isomorphism(default_into = 10u8)]`
      - When **convert** is invoked but a variant does not have specified convert-into value, this value will be matched. If this one is not given either, a default value of convert-into type will be used (The type must have Default then).

    - `#[isomorphism(skip_restore)]`
      - Do not implement neither `From<(&)OtherType>` for `Self`

    - `#[isomorphism(skip_ref_restore)]`
      - Do not implement `From<&OtherType>` for `Self`

    - `#[isomorphism(restore_panic = "panic..!")]`
      - In the `From<(&)OtherType>` for `Self`'s match syntax, the rest cases will invoke panic with the given panic literal: `_ => panic!("panic..!")`.
    - `#[isomorphism(default_restore = SelfValue)]`
      - In the `From<(&)OtherType>` for `Self`'s match syntax, if the `restore_panic` is not given, this value will be used for rest cases's match: `_ => SelfValue`.
      - If `default_restore` is not given either, the default value of Self type will be used: `_ => Self::default()`. (Then it must impl Default)

- _Variant Attributes_
  - `#[from_variant]` / `#[from_variant(skip)]`
    - Variants with `#[from_variant]` attribute still invoke **from_variant** regardless of the container attribute.
    - However ones with `#[from_variant(skip)]` do not invoke **from_variant** implements for themselves even though the container attribute has `from_variant`.
  - `#[name("specific_var_name")]`
    - Specify the variant's matching name literal for **name_variant**'s method.
  - `#[into(10u8)]`
    - Specify the variant's convert-into value for **convert**.
    - If it's not given, container attribute's `default_into` value - or, without it, the type's default value - will be used.
  - `#[restore(SelfValue)]`
    - Specify the variant's value that will be converted from its matching convert-into value. (In `From<(&)OtherType>` for `Self`).
    - If it's not given, the default value for the variant will be used.

### _Examples_

```rust
use seoul::Isomorphism;

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
```

```rust
use seoul::Isomorphism;

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
```

## Trait Reflica

Declare a borrowed fields' data type ("reflica") from an original struct/enum data type, and implement Into trait to the reflica.

### _Abilities_:

- `struct AB { a: u8, b: String }` ->
  - declare `struct RefAB<'a> { a: &'a u8, b: &'a String }`
  - impl `Into<RefAB<'a>>` for `&'a AB`
- `enum AB { A, B { a: u8, b: String } }` ->
  - declare `enum RefAB<'a> { A, B { a: &'a u8, b: &'a String } }`
  - impl `Into<RefAB<'a>>` for `&'a AB`

### _Attributes_

- _Container Attributes_
  - `#[reflica(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]`
    - The traits listed will be passed to the `#[derive(...)]` attribute of the new Reflica data type.
    - In the `reflica` attribute, any path with name other than `prefix` will be counted as a trait.

  - `#[reflica(prefix="SpecificPrefix")]`
    - By default, the new Reflica type's name is determined by `Ref` + `Original type's name`. However other prefix name can be passed with the `prefix` path.

### _Example_

```rust
use seoul::Reflica;

// struct
#[derive(Reflica)]
// attribute for derive implementation for the reflica
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
```
