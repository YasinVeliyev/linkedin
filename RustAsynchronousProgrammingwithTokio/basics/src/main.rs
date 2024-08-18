use std::{thread, time};
use tokio::task;
use tokio::time::{sleep, Duration};
async fn hello(name: &str) -> String {
    format!("Hello {name}")
}

fn blocking_call() -> String {
    thread::sleep(time::Duration::from_secs(5));
    "Finally done it".to_string()
}

async fn async_call(id: i32) {
    sleep(Duration::from_secs(1)).await;
    println!("Asycn Call Id {}", id)
}

#[tokio::main]
async fn main() {
    let join_handel = tokio::spawn(hello("Yasin"));
    let value = join_handel.await.unwrap();
    println!("{value}");

    let blocking_call_handle = tokio::task::spawn_blocking(blocking_call);
    let mut async_handles = Vec::new();

    for i in 0..10 {
        async_handles.push(tokio::spawn(async_call(i)));
    }

    for handle in async_handles {
        handle.await.unwrap();
    }

    println!("{}", blocking_call_handle.await.unwrap());
}
