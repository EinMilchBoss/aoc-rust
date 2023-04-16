use std::{
    sync::{mpsc, Arc, RwLock},
    thread,
};

fn solve_first(input: &str) -> String {
    let input = Arc::new(RwLock::new(input.to_string()));
    let found_result = Arc::new(RwLock::new(false));
    let (tx, rx) = mpsc::channel::<(usize, String)>();

    let result_handle = thread::spawn({
        let found_result = Arc::clone(&found_result);
        move || {
            while let Ok((counter, hash)) = rx.recv() {
                if hash.starts_with("00000") {
                    *found_result.write().unwrap() = true;
                    return counter;
                }
            }
            panic!("No hash could be found that fulfills the requirements.");
        }
    });

    let mut counter = 1;
    loop {
        thread::spawn({
            let input = Arc::clone(&input);
            let tx = tx.clone();
            move || {
                let secret_key = format!("{}{}", input.read().unwrap(), counter);
                let hash = format!("{:x}", md5::compute(secret_key));
                let _ = tx.send((counter, hash));
            }
        });

        if *found_result.read().unwrap() {
            break;
        }

        counter += 1;
    }

    result_handle.join().unwrap().to_string()
}

fn solve_second(input: &str) -> String {
    let input = Arc::new(RwLock::new(input.to_string()));
    let found_result = Arc::new(RwLock::new(false));
    let (tx, rx) = mpsc::channel::<(usize, String)>();

    let result_handle = thread::spawn({
        let found_result = Arc::clone(&found_result);
        move || {
            while let Ok((counter, hash)) = rx.recv() {
                if hash.starts_with("000000") && hash.chars().nth(6).unwrap() != '0' {
                    *found_result.write().unwrap() = true;
                    return counter;
                }
            }
            panic!("No hash could be found that fulfills the requirements.");
        }
    });

    let mut counter = 1;
    loop {
        thread::spawn({
            let input = Arc::clone(&input);
            let tx = tx.clone();
            move || {
                let secret_key = format!("{}{}", input.read().unwrap(), counter);
                let hash = format!("{:x}", md5::compute(secret_key));
                let _ = tx.send((counter, hash));
            }
        });

        if *found_result.read().unwrap() {
            break;
        }

        counter += 1;
    }

    result_handle.join().unwrap().to_string()
}

fn main() {
    let example = "abcdef";
    let input = "ckczppom";

    println!("First: Expected {} found {}.", 609043, solve_first(example));
    println!(
        "Second: Expected {} found {}.",
        18549057,
        solve_second(example)
    );

    println!("First: {}", solve_first(input));
    println!("Second: {}", solve_second(input));
}
