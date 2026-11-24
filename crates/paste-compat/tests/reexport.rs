//! The shim must be usable exactly like `paste`: same macro names, same syntax.

paste::paste! {
    fn [<answer_ 42>]() -> u32 {
        42
    }

    #[test]
    fn paste_macro_is_reexported() {
        assert_eq!(answer_42(), 42);
    }
}

paste::item! {
    fn [<item_ macro>]() -> u32 {
        7
    }

    #[test]
    fn item_macro_is_reexported() {
        assert_eq!(item_macro(), 7);
    }
}
