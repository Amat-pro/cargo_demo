use std::{thread, time::Duration};

#[derive(Debug)]
struct Ignore {
    a: u8,
}

#[derive(Debug)]
struct TestIgnore {
    a: u8,
    b: bool,
    ignore: Ignore,
}

struct Server {
    a: u8,
    b: bool,
}

#[cfg(test)]
mod tests {
    // use futures::executor::block_on;
    use std::thread;
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_ignore() {
        let a = 2;
        let b = true;
        let ignore = Ignore { a: 3 };
        let ignore_zero = TestIgnore { a: a, b: b, ignore };

        println!(
            "===>>> {} {} {}",
            ignore_zero.a, ignore_zero.b, ignore_zero.ignore.a
        );
        println!("===>>> {:?}", ignore_zero);

        let TestIgnore { a, b, ignore } = ignore_zero;
        // println!("===>>> {:?}", ignore_zero);
    }

    #[test]
    fn test_server_loop() {
        println!("===========================================");

        let s = Server { a: 1, b: true };

        thread::sleep(Duration::from_secs(10));

        println!("remove s");
        let Server { a: a, b: b } = s;

        println!("{}", a);
        println!("{}", b);
        println!("sleep 10 sec");
        thread::sleep(Duration::from_secs(10));
        println!("done");

        println!("===========================================");
    }
}
