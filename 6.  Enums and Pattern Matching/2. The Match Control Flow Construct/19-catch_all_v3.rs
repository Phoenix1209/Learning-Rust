fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}

/*	Here, we’re telling Rust explicitly that we aren’t going to use any other value that doesn’t
	match a pattern in an earlier arm, and we don’t want to run any code in this case.
*/