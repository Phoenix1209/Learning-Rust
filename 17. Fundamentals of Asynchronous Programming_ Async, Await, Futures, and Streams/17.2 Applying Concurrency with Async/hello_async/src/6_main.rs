extern crate trpl; // required for mdbook test

use std::thread;
use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let fut1 = async {
            for i in 1..5 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = {
            for i in 1..10 {
                println!("hi number {i} from the second task!");
			    thread::sleep(Duration::from_millis(500));
            }
        };

		fut1.await;
    });
}