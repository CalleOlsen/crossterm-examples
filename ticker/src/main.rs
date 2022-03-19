use std::{error::Error, time::SystemTime};

use tokio::time::{self, interval};

#[tokio::main]
async fn main() {
    let mut interval = time::interval(time::Duration::from_millis(20));
    let mut x = 0;

    loop {
        interval.tick().await;
        my_task(x);
        x += 1;
    }

    println!("Hello, world!");
}

fn my_task(tick: u32) {
    let now = SystemTime::now();

    match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            println!("Tick <{}>  Since epoc [{}]", tick, n.as_secs());
        }
        Err(_) => {
            println!("Error , unable to read system time");
        }
    }
}
