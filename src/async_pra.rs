use std::{future::IntoFuture, thread, time::Duration};

pub async fn challenge1() -> i32 {
    let one = async {
        thread::sleep(Duration::from_secs(2));
        println!("finish");
        9
    };

    println!("finish");
    one.await
}