// Listing 11-13: An integration test of a function in the adder crate

use adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}