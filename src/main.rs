use oink::sentence_to_pig_latin;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let sentence = &*args[1];
    match sentence_to_pig_latin(sentence) {
        Some(sentence) => {
            println!("\n{}", sentence);
        }
        None => {
            println!("\nCouldn't convert string to pig latin.");
        }
    }
}
