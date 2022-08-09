// this is a lib

// macros
// @lib: quote, syn
// @link https://doc.rust-lang.org/book/ch19-06-macros.html
// macro frag spec: @link https://doc.rust-lang.org/reference/macros-by-example.html
// Mater Rust: @see https://doc.rust-lang.org/stable/reference/macros.html

// 1. declarative macro
// @Example: manifest-dir-macros(github)
// cannot export: `proc-macro` crate types currently cannot export any items other than functions tagged with `#[proc_macro]`, `#[proc_macro_derive]`, or `#[proc_macro_attribute]`
// @see macro_demo_trait declarative_macro
// macro_rules! declarative_macro {
//     () => {
//         "==> declarative macro: ()"
//     };
//
//     // any: *
//     // at least one: +
//     // zero or any: ?
//
//     // example: [1, 2]
//     ($($elem:expr)+) => {
//         "==> declarative macro: $($elem:expr)+"
//     };
//
//     // example: (1, 2)
//     ($($elem:expr), +) => {
//         "==> declarative macro: $($elem:expr), +)"
//     };
//
// }
//
// #[cfg(test)]
// mod test_declarative_macro {
//     #[test]
//     fn test_declarative_macro() {
//         println!(declarative_macro!());
//         println!(declarative_macro!(0, 1));
//         println!(declarative_macro!([0, 1]));
//     }
// }

// 2. proc macro
// @Note: @link https://stackoverflow.com/questions/56713877/why-do-proc-macros-have-to-be-defined-in-proc-macro-crate
use proc_macro::TokenStream;
use quote::quote;
use syn;

// 2.1 Function-like macro
// can't use a procedural macro from the same crate that defines it
#[proc_macro]
pub fn sql2(input: TokenStream) -> TokenStream {
    input
}

// 2.2 Derive Macro
#[proc_macro_derive(MyDeriveMacroTrait)]
pub fn hello_derive_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_derive_macro(&ast)
}

fn impl_hello_derive_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl MyDeriveMacroTrait for #name {
            fn hello_derive_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

// 2.3 Attribute-like macro
// expected unsuffixed literal or identifier, found `MyDeriveMacroTrait`
#[proc_macro_attribute]
pub fn route(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}


