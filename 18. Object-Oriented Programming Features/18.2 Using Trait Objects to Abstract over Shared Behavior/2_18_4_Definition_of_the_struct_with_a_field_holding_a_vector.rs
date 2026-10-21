// Listing 18-4: Definition of the Screen struct with a components field holding a vector of trait objects that implement the Draw trait

// This code is a library crate

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}