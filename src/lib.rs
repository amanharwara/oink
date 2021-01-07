//! `oink` is a library & command-line tool that allows you to translate from English to Pig Latin.
//!
//! ## How to use as a library
//!
//! ```
//! use oink::{word_to_pig_latin, sentence_to_pig_latin};
//!
//! // Convert a single word to pig latin & print it.
//! match word_to_pig_latin("Word") {
//!     Some(word) => {println!("{}", word)}
//!     None => {println!("Word")}
//! }
//!
//! // Convert a sentence/paragraph(s) to pig latin & print it.
//! match sentence_to_pig_latin("This is a sentence.") {
//!     Some(sentence) => {println!("{}", sentence)}
//!     None => {println!("This is a sentence.")}
//! }
//! ```
//!
//! ## How to use as a command-line tool
//!
//! Install using `cargo`:
//!
//! ```
//! cargo install oink
//! ```
//!
//! Use command:
//!
//! ```
//! oink <STRING>
//! ```

#[macro_use]
extern crate lazy_static;
use regex::Regex;

/// Capitalize a word i.e. make the first letter of a word uppercase.
fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Translates a single word from English to Pig Latin.
///
/// # Examples
///
/// ```
/// let translated = oink::word_to_pig_latin("Consequence").unwrap();
///
/// assert_eq!("Onsequencecay", &translated);
/// ```
pub fn word_to_pig_latin(word: &str) -> Option<String> {
    lazy_static! {
        static ref IS_VOWEL: Regex = Regex::new(r"(?i)[aeiou]").unwrap();
        static ref IS_CONSONANT: Regex = Regex::new(r"(?i)[^aeiou]+").unwrap();
    }
    if word.len() == 0 {
        return None;
    }
    let word = word;
    if IS_VOWEL.is_match(&word[0..1]) {
        Some(format!("{}yay", {
            if word.chars().next().unwrap().is_uppercase() {
                capitalize(word)
            } else {
                word.to_string()
            }
        }))
    } else {
        let mat = IS_CONSONANT.find(&word).unwrap();
        Some(format!(
            "{}{}ay",
            {
                let consonant = &word[0..mat.end()];
                let other = &word[mat.end()..];
                if consonant.chars().next().unwrap().is_uppercase() {
                    capitalize(&word[mat.end()..])
                } else {
                    other.to_string()
                }
            },
            &word[0..mat.end()].to_lowercase()
        ))
    }
}

/// Translates a sentence from English to Pig Latin.
///
/// # Examples
///
/// ```
/// let translated = oink::sentence_to_pig_latin("Hello, my name is Scott Walker.").unwrap();
///
/// assert_eq!("Ellohay, myay amenay isyay Ottscay Alkerway.", &translated);
/// ```
pub fn sentence_to_pig_latin(sentence: &str) -> Option<String> {
    if sentence.len() == 0 {
        return None;
    }

    let is_word = Regex::new(r"(\w+'?\w*)").unwrap();
    let words = is_word.find_iter(sentence);
    let mut sentence = sentence.to_string();
    for word in words {
        let reg = format!(r"\b{}\b", word.as_str());
        let word_regex = Regex::new(&reg).unwrap();
        let pig = word_to_pig_latin(word.as_str());
        match pig {
            Some(pig) => {
                let pig_ref = pig.as_str();
                let replaced = word_regex.replacen(&sentence, 1, pig_ref);
                sentence = replaced.to_string();
            }
            None => {}
        }
    }

    Some(sentence)
}

#[cfg(test)]
mod tests {
    use crate::{sentence_to_pig_latin, word_to_pig_latin};

    #[test]
    fn to_pig_consonant() {
        assert_eq!(word_to_pig_latin("Hello").unwrap(), "ellohay");
    }

    #[test]
    fn to_pig_consonant_cluster() {
        assert_eq!(word_to_pig_latin("Smile").unwrap(), "ilesmay");
    }

    #[test]
    fn to_pig_vowel() {
        assert_eq!(word_to_pig_latin("Aman").unwrap(), "amanyay");
    }

    #[test]
    fn to_pig_sentence() {
        assert_eq!(
            sentence_to_pig_latin("Hello, I am Aman").unwrap(),
            "ellohay, iyay amyay amanyay"
        );
    }
}
