pub trait MyDeriveMacroTrait {
    fn hello_derive_macro();
}

#[macro_export]
macro_rules! declarative_macro {
    () => {
        "==> declarative macro: ()"
    };

    // any: *
    // at least one: +
    // zero or any: ?

    // example: [1, 2]
    ($($elem:expr)+) => {
        "==> declarative macro: $($elem:expr)+"
    };

    // example: (1, 2)
    ($($elem:expr), +) => {
        "==> declarative macro: $($elem:expr), +)"
    };

}

#[cfg(test)]
mod test_declarative_macro {
    #[test]
    fn test_declarative_macro() {
        println!(declarative_macro!());
        println!(declarative_macro!(0, 1));
        println!(declarative_macro!([0, 1]));
    }
}
