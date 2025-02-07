/*
If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8
values that looks like this:

[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode scalar
values, which are what Rust’s char type is, those bytes look like this:

['न', 'म', 'स', '्', 'त', 'े']

There are six char values here, but the fourth and sixth are not letters: they’re diacritics that don’t
make sense on their own. Finally, if we look at them as grapheme clusters, we’d get what a person would
call the four letters that make up the Hindi word:

["न", "म", "स्", "ते"]

Rust provides different ways of interpreting the raw string data that computers store so that each
program can choose the interpretation it needs, no matter what human language the data is in.
*/