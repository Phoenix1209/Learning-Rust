fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let mut msg = Message::Hello { id: 5 };

	println!("msg = {:?}", msg);

    match msg {
        Message::Hello.id = 3..=7  => {
            println!("Found an id in range: {id}")
        }
		/*
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
		*/
    }
}