struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn main() {}

/*	But if we used the different structs, each of which has its own type,
	we couldn’t as easily define a function to take any of these kinds of messages as
	we could with the Message enum defined in Listing 6-2, which is a single type. */