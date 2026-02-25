fn main() {
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}

/*
The reason is that types such as integers that have a known size at compile time are stored
entirely on the stack, so copies of the actual values are quick to make. That means there’s
no reason we would want to prevent x from being valid after we create the variable y. In other
words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do
anything different from the usual shallow copying, and we can leave it out.
*/