use async_pra::challenge1;
use futures::join;
use std::{thread, time::Duration};

use tokio;

mod fib;
mod async_pra;


async fn hello_world() {
    println!("hello world")
}

async fn hello_world2() {
    println!("hello world2")
}

async fn run() {
    join!(hello_world(), hello_world2());
}

#[tokio::main]
async fn main() {
    println!("1");
    let handle = tokio::spawn(async {
        println!("2");
        thread::sleep(Duration::from_secs(5));
        println!("4");
        "hello world".to_string()
    });
    
    let handle2 = tokio::spawn(async {
        thread::sleep(Duration::from_secs(2));
        println!("3");
        "good after noon".to_string()
    });

    // handle の実行結果は async ブロックの値になります
    let result: String = handle.await.unwrap();
    println!("{}", result);
    let result: String = handle2.await.unwrap();
    println!("{}", result);
}
