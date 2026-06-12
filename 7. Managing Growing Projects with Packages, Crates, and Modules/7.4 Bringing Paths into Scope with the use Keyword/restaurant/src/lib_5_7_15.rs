// Listing 7-15: Bringing two types with the same name into the same scope requires using
// their parent modules.

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}