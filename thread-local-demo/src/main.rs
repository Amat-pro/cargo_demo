use std::cell::RefCell;
use std::thread;

fn main() {
    // 1. thread local of std
    thread_local! {static FOO: RefCell<u32> = RefCell::new(1)}

    FOO.with(|f| {
        println!("1. f.borrow is {}", *f.borrow());
        *f.borrow_mut() = 3;
        println!("2. f.borrow is {}", *f.borrow());
    });

    let t = thread::spawn(move || {
        FOO.with(|f| {
            println!("3. f.borrow is {}", *f.borrow());
        });
    });

    t.join().unwrap();

    // 2. use lib for thread-local
    use thread_local::ThreadLocal;
    use std::sync::Arc;
    use std::cell::Cell;

    let tls = Arc::new(ThreadLocal::new());
    for _ in 0..5 {
        let tls2 = tls.clone();
        thread::spawn(move || {
            let cell = tls2.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
        }).join().unwrap();
    }

    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| x + y.get());

    println!("4. total is {}", total);
}

