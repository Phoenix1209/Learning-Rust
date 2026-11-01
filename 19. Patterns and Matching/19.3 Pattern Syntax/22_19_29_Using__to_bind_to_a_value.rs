// Listing 19-29: Using @ to bind to a value in a pattern while also testing it

fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}")
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}

/*
error[E0425]: cannot find value `id` in this scope

Message::Hello { id: 10..=12 } => {
    println!("id: {id}");
}

Un rango o literal en un patrón solo prueba el valor y no
lo liga a ningún nombre. Para usarlo dentro del brazo
necesitas @ (id: id_variable @ 10..=12) o un patrón que lo
capture ({ id }).
*/