// Listing 18-1: An AveragedCollection struct that maintains a list of integers and the average of the items in the collection

// This code is a library crate

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}