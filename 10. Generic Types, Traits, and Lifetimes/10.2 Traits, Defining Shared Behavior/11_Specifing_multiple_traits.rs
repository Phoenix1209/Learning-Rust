// Different parameters
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

// Same parameters
pub fn notify<T: Summary>(item1: &T, item2: &T) {}