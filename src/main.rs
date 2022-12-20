mod practice;

use std::collections::HashMap;

use lazy_static::lazy_static;

use macro_demo::route;
use macro_demo::sql2;
use macro_demo::MyDeriveMacroTrait;
use macro_demo_trait::declarative_macro;
use macro_demo_trait::MyDeriveMacroTrait;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

fn main() {
    // First: declarative macro
    println!(declarative_macro!());
    println!(declarative_macro!(0, 1));
    println!(declarative_macro!([0, 1]));
    println!(declarative_macro![0, 1]);

    // Second: proc macros

    // 2.1. Function-like macro
    println!(sql2!("sql 1"));
    sql2! {
        println!("sql2");
    }
    test();

    // 2.2 Derive Macro
    #[derive(MyDeriveMacroTrait)]
    struct DeriveMacroTestStruct;
    DeriveMacroTestStruct::hello_derive_macro();

    // 2.3. Attribute-like macro
    index();

    // a lazy_static demo
    lazy_static_do();
}

// 1. Function-like macro
sql2!(
    trait Example {
        const CONST_NO_DEFAULT: i32;
        const CONST_WITH_DEFAULT: i32 = 99;
        type TypeNoDefault;
        fn method_without_default(&self);
        fn method_with_default(&self) {}
    }
);
sql2! {
    fn test() {
        println!("hello, i am test")
    }
}

// 3. Attribute-like macro
#[route(GET, "/")]
pub fn index() {
    println!("hello, i am a attribute-like macro")
}

fn lazy_static_do() {
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
}
