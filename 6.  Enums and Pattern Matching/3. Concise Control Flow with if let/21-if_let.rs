fn main() {
    let config_max = Some(3u8);
    // if let mut Some(max) = config_max {	// mut no funciona en este caso
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}