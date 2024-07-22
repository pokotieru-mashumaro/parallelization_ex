use std::{sync::mpsc, thread, time::Instant};

fn parallelization() {
    let nums = [43, 111, 3, 90, 7];
    let start_time = Instant::now();

    let (tx, rx) = mpsc::channel::<(i64, i64)>();
    for num in nums {
        let sender = tx.clone();
        thread::spawn(move || loop {
            let ans = fib();
        });
    }
}

fn fib(n: i64) ->i64 {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    return fib(n - 2) + fib(n - 1);
}