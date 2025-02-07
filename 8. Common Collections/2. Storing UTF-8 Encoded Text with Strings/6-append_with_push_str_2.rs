fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
	
	println!("s1 is: {s1}");
    println!("s2 is: {s2}");
}

/* If the push_str method took ownership of s2, we wouldn’t be able to print its value on the last line.
However, this code works as we’d expect! */