// Listing 19-10: Attempting to use an irrefutable pattern with let...else

fn main() {
    let x = 5 else {
        return;
    };
}