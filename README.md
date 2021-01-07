# oink

Oink is a English -> Pig Latin translator library & command-line tool written in Rust.

## Use as Rust library

```rust
use oink::{word_to_pig_latin, sentence_to_pig_latin};

// Convert a single word to pig latin & print it.
match word_to_pig_latin("Word") {
    Some(word) => {println!("{}", word)}
    None => {println!("Word")}
}

// Convert a sentence/paragraph(s) to pig latin & print it.
match sentence_to_pig_latin("This is a sentence.") {
    Some(sentence) => {println!("{}", sentence)}
    None => {println!("This is a sentence.")}
}
```

## Use as command-line tool

Install using `cargo`:

```
cargo install oink
```

Use command:

```
oink <STRING>
```
