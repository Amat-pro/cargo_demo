use std::fs::File;
use std::{time, thread};
use std::io::Write;
use pprof::protos::Message;

pub fn do_something() {
    let prime_numbers = prepare_prime_numbers();

    let guard = pprof::ProfilerGuard::new(1000).unwrap();

    let mut v = 0;

    for i in 2..5_000_000 {
        if i % 4 == 0 {
            if is_prime_number1(i, &prime_numbers) {
                v += 1;
            }
        } else if i % 4 == 1 {
            if is_prime_number2(i, &prime_numbers) {
                v += 1;
            }
        } else {
            if is_prime_number3(i, &prime_numbers) {
                v += 1;
            }
        }

        println!("Prime numbers: {}", v);
    }

    //
    println!("===> report first");
    if let Ok(report) = guard.report().build() {
        let mut file = File::create("profile.pb").unwrap();
        let profile = report.pprof().unwrap();

        let mut content = Vec::new();

        profile.write_to_vec(&mut content).unwrap();
        file.write_all(&content).unwrap();

        println!("report: {:?}", report);
    }

    // sleep twenty seconds and report again
    let twenty_secs = time::Duration::from_secs(20);
    thread::sleep(twenty_secs);
    println!("===> report second");
    if let Ok(report) = guard.report().build() {
        let mut file = File::create("profile-second.pb").unwrap();
        let profile = report.pprof().unwrap();

        let mut content = Vec::new();

        profile.write_to_vec(&mut content).unwrap();
        file.write_all(&content).unwrap();

        println!("report: {:?}", report);
    }
}

fn prepare_prime_numbers() -> Vec<usize> {
    let mut prime_number_table: [bool; 10_000] = [true; 10_000];
    prime_number_table[0] = false;
    prime_number_table[1] = false;

    for i in 2..10_000 {
        if prime_number_table[i] {
            let mut v = i * 2;
            while v < 10_000 {
                prime_number_table[v] = false;
                v += 1;
            }
        }
    }

    let mut prime_numbers = vec![];
    for (i, exist) in prime_number_table.iter().enumerate().skip(2) {
        if *exist {
            prime_numbers.push(i);
        }
    }

    prime_numbers
}

fn is_prime_number1(v: usize, prime_numbers: &[usize]) -> bool {
    if v < 10_000 {
        let search_result = prime_numbers.binary_search(&v);
        return search_result.is_ok();
    }

    for n in prime_numbers {
        if v % n == 0 {
            return false;
        }
    }

    true
}

fn is_prime_number2(v: usize, prime_numbers: &[usize]) -> bool {
    if v < 10_000 {
        let search_result = prime_numbers.binary_search(&v);
        return search_result.is_ok();
    }

    for n in prime_numbers {
        if v % n == 0 {
            return false;
        }
    }

    true
}

fn is_prime_number3(v: usize, prime_numbers: &[usize]) -> bool {
    if v < 10_000 {
        let search_result = prime_numbers.binary_search(&v);
        return search_result.is_ok();
    }

    for n in prime_numbers {
        if v % n == 0 {
            return false;
        }
    }

    true
}