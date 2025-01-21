fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("s: {s}");
	println!("s1: {s1}");
	println!("s2: {s2}");
	println!("s3: {s3}");
}