use std::{sync::mpsc, thread, time::{Duration, Instant}};

pub fn normal() {
    let nums = [43, 29, 39, 20, 42];
    let start_time = Instant::now();

    for num in nums {
        let ans = fib(num);
        println!("結果: fib ({})={}", num, ans);
    }
    showtime(start_time);
}

pub fn parallelization() {
    let nums = [43, 29, 39, 20, 42];
    let start_time = Instant::now();

    let (tx, rx) = mpsc::channel::<(i64, i64)>();
    for num in nums {
        let sender = tx.clone();
        thread::spawn(move || {
            let ans = fib(num);
            sender.send((num, ans)).unwrap();
        });
    }

    let mut job = nums.len();
    loop {
        if let Ok((arg, ans)) = rx.recv() {
            job -= 1;
            println!("結果: fib ({})={} (残り{})", arg, ans, job);
            if job <= 0 {
                showtime(start_time);
                break;
            }
        }
        thread::sleep(Duration::from_millis(300));
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

fn showtime(start_time: Instant) {
    let time = start_time.elapsed();
    println!("実行時間: {:?}", time);
}