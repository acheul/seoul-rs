# Version Change Log

- ✈️ ver 1.0.0
    - Merged traits: Keep only `Isomorphism` and `Reflica`.
    - `Reflica` remains uncahged.
    - `Isomorphism` has major changes:
        * Former `IntoWrap` and `Tuplike` traits' abilities are merged into `Isomorphism`.
            - `IntoWrap` -> `Isomorphism`'s "from_variant"
            - `Tuplike` -> `Isomorhpism`'s "intofrom_tuple"
        * Added "into_field" ability.
            - Convert Self into each field's type.
        * Several changes in former `Isomorphism`'s attribute syntax:
            - "title" method -> "name"
            - "list" method -> Removed.
            - "into" (convert) ability => More organized overall.

---

- ver 0.3.7
    - On `IntoWrap`: now works on not only Enum but also Struct datas which have just one field.
- ver 0.3.6
    - crate `seoul-derive`:
        - a bit revision of error comments;
    - On `Reflica`: copy visibility of a type and fields; (pub, pub(crate), etc.)
- ver 0.3.5
    - On `Isomorphism`:
        - Reinforced derive macro's generic parsing ability.
        - For struct type, the derive macro would only implement the Isormophism trait with each methods of it returning default values.

- ver 0.3.3
    - On `Tuplike`:
        - delete associated type `Tuple` from the trait.
        - derive macro now works on the enum type too: but only `From<T>` trait will be implemented for each variant.
    - Add `IntoWrap`
- ver 0.3.2
    - Add `Reflica`
- ver 0.3.1
    - Add an associated type `Tuple` to the trait `Tuplike`
- ver 0.3.0
    - Add `Tuplike`.

- ver 0.2.1~2
    - correct some typos
- ver.0.2.0
    - `From<T>` implemented only with `has_default` attribute syntax when `Default` is implemented.

---