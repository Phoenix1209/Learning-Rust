let text = "Hola   mundo\n desde\tRust";
let iter = text.split_whitespace();

for word in iter {
    println!("{}", word);
}

// Salida
/*
Hola
mundo
desde
Rust
*/

let text = "Hola   mundo\n desde\tRust";
let words: Vec<&str> = text.split_whitespace().collect();
println!("{:?}", words);

// Salida
/*
["Hola", "mundo", "desde", "Rust"]
*/

let text = "Hola   mundo\n desde\tRust";
let words: Vec<String> = text
    .split_whitespace()
    .map(|word| word.to_string()) // Convertimos cada &str en String
    .collect();
println!("{:?}", words);

// Salida
/*
["Hola", "mundo", "desde", "Rust"]
*/