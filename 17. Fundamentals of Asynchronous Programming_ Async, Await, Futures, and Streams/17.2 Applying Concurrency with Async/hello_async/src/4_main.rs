extern crate trpl; // required for mdbook test

use std::thread;
use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let fut1 = {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                thread::sleep(Duration::from_millis(500));
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        fut2.await;
    });
}