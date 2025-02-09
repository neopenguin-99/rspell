#![feature(custom_test_frameworks)]
#![feature(test)]
#![allow(arithmetic_overflow)]

use rspell::rspell::word_reader::word_reader::get_words_from_file_as_tree;
use unicode_segmentation::UnicodeSegmentation;
use std::{cmp::min, io::Read};
use std::fs::File;
use distance::levenshtein;

use clap::{crate_authors, crate_version, value_parser, Arg, ArgMatches, Command};
use rspell::rspell::word_reader::get_words_from_file;
use rspell::rspell::word_reader::Node;

/// Returns a vector of String that are the relative paths of the files passed as arguments.
fn parse_args() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let args: ArgMatches = Command::new("rspell")
        .version(crate_version!())
        .author(crate_authors!("\n"))
            .arg(Arg::new("files")
            .num_args(0..)
            .value_parser(value_parser!(String))
        )
        .try_get_matches()?;
    let files_option = args.get_many::<String>("files");
    return match files_option {
        Some(files) => Ok(files.map(|s| s.to_string()).collect::<Vec<String>>()),
        None => panic!("No file name was provided"),
    };
}




fn compute_levenshtein_distance(s: &str, t: &str) -> u32 {
    if s.is_empty() { return t.len() as u32; }

    if t.is_empty() { return s.len() as u32; }
    unsafe {
        if s.get_unchecked(0..1) == t.get_unchecked(0..1) {
            return compute_levenshtein_distance(s.get(1..).unwrap_or(""), t.get(1..).unwrap_or(""))
        }
        else {
            let vec = vec![
                compute_levenshtein_distance(s.get(1..).unwrap_or(""), t),
                compute_levenshtein_distance(s, t.get(1..).unwrap_or("")),
                compute_levenshtein_distance(s.get(1..).unwrap_or(""), t.get(1..).unwrap_or(""))
            ];
            return *vec.iter().min().unwrap() + 1;
        }
    }
}

fn compute_levenshtein_distance_iterative_full_matrix(s: &str, t: &str) -> u32 {
    let mut levenshtein_distance_vec: Vec<Vec<u32>> = Default::default();
    // populate vec with '0' values
    let mut i = 0;
    while i < s.len() {
        levenshtein_distance_vec.push(vec![]);
        let mut j = 0;
        while j < t.len() {
            levenshtein_distance_vec.get_mut(i).unwrap().push(0);
            j = j + 1;
        }
        i = i + 1;
    }

    let mut i = 1;
    // println!("{:#?}", levenshtein_distance_vec);
    while i < s.len() {
        levenshtein_distance_vec.push(vec![i as u32]);
        i = i + 1;
    }
    // println!("{:#?}", levenshtein_distance_vec);
    let mut j = 1;
    while j < t.len() {
        levenshtein_distance_vec[0].push(j as u32);
        j = j + 1;
    }
    levenshtein_distance_vec[0].dedup();
    // println!("{:#?}", levenshtein_distance_vec);

    let mut j = 1;
    while j < t.len() {
        let mut i = 1;
        let mut substitution_cost;
        while i < s.len() {
            if UnicodeSegmentation::graphemes(s, true).nth(0) == UnicodeSegmentation::graphemes(t, true).nth(0) {
                substitution_cost = 0;
            }
            else {
                substitution_cost = 1;
            }
            println!("for i is {i} and j is {j}");
            if j == 1 {
                levenshtein_distance_vec.push(vec![]);
            }

            let a = levenshtein_distance_vec.get(i - 1).unwrap().get(j).unwrap().clone(); //deletion
            let b = levenshtein_distance_vec.get(i).unwrap().get(j - 1).unwrap().clone(); //insertion
            let c = levenshtein_distance_vec.get(i - 1).unwrap().get(j - 1).unwrap().clone() + substitution_cost; //substitution
            let current: &mut Vec<u32> = levenshtein_distance_vec.get_mut(i - 1).unwrap();
            // println!("before push: {:#?}", current);
            current.push(min(
                a, min(b, c)
            ));
            // println!("after push: {:#?}", current);
            // let a = levenshtein_distance_vec.push(current);
            i = i + 1;
        }
        j = j + 1;
    }
    // println!("{:#?}", levenshtein_distance_vec);
    return levenshtein_distance_vec.get(s.len() - 1).unwrap().get(t.len() - 1).unwrap().clone();
}

enum Spelling {
    Correct,
    Incorrect(String, String, usize)
}

fn check_word_against_dictionary(word: &String, dictionary: &Vec<String>) -> Result<Spelling, Box<dyn std::error::Error>> {
    let mut min_distance: usize = usize::MAX;
    let closest_word: String = Default::default();
    for dict_entry in dictionary {
        let distance = levenshtein(&word, &dict_entry); // todo use di so that we
        if distance < min_distance {
            min_distance = distance
        }
    }
    if min_distance == 0 {
        return Ok(Spelling::Correct);
    }
    else {
        return Ok(Spelling::Incorrect(word.to_string(), closest_word, min_distance));
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = parse_args()?;
    let dictionary = get_words_from_file("training2.txt")?;
    let dictionary = get_words_from_file_as_tree("training2.txt")?;
    for file in files {
        println!("For file {}:", file.to_string());
        let words = get_words_from_file(&file)?;
        for word in words {
            println!("checking word:{}", word);
            let spelling = check_word_against_dictionary(&word, &dictionary)?;
            _ = match spelling {
                Spelling::Correct => { 
                    continue; 
                }
                Spelling::Incorrect(incorrect_word, closest_word, distance) => {
                    println!("{incorrect_word} -> {closest_word} ({distance})");
                }
            }
        }
    }
    Ok(())
}

fn traverse_and_retrieve_closest_word(root_node: Node<String>) -> Result<String, Box<dyn std::error::Error>> {
    let a = root_node.children;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::get_words_from_file;
    use assert_fs::prelude::*;
    use mockall::{automock, mock, predicate::*};
    use test_case::test_case;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_get_words_from_file() -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        const FILE_NAME: &str = "get_words_from_file.txt";
        let temp = assert_fs::TempDir::new()?;
        let input_file = temp.child(FILE_NAME);
        input_file.touch()?;
        input_file.write_str("this statement is false")?;
        let file_path = input_file.path().to_str().unwrap();

        // Act
        let result = get_words_from_file(file_path)?;

        // Assert
        assert_eq!(result, vec!["this", "statement", "is", "false"]);

        // Teardown
        temp.close().unwrap();
        Ok(())

    }

    #[test_case("worr", "word", 1 ; "when distance is 1 from 1 substituted character")]
    #[test_case("wirr", "word", 2 ; "when distance is 1 from 2 substituted characters")]
    #[test_case("wor", "word", 1 ; "when distance is 1 from 1 deleted character")]
    #[test_case("worda", "word", 1 ; "when distance is 1 from 1 added character")]
    fn compute_levenshtein_distance(incorrect_word: &str, closest_match: &str, expected_distance: u32) -> Result<(), Box<dyn std::error::Error>> {
        let distance = super::compute_levenshtein_distance(incorrect_word, closest_match);
        assert_eq!(distance, expected_distance);
        Ok(())
    }

    #[test]
    fn check_word_against_dictionary_returns_correct_spelling_enum_variant() -> Result<(), Box<dyn std::error::Error>> {
        let distance = super::check_word_against_dictionary(&String::from("word"), &vec![String::from("dictionary"), String::from("with"), String::from("word")])?;
        assert!(match distance {
            Spelling::Correct => true,
            Spelling::Incorrect(_, _, _) => false
        });
        Ok(())
    }

    #[test]
    fn check_word_against_dictionary_returns_incorrect_spelling_enum_variant() -> Result<(), Box<dyn std::error::Error>> {
        let distance = super::check_word_against_dictionary(&String::from("worf"), &vec![String::from("dictionary"), String::from("with"), String::from("word")])?;
        assert!(match distance {
            Spelling::Incorrect(_, _, _) => true,
            Spelling::Correct => false
        });
        Ok(())
    }

    #[bench]
    fn compute_levenshtein_distance_performance(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        b.iter(|| {
            _ = super::compute_levenshtein_distance("worr", "word");
        });
        Ok(())
    }

    #[bench]
    fn compute_levenshtein_distance_iterative_full_matrix_performance(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        b.iter(|| {
            _ = super::compute_levenshtein_distance_iterative_full_matrix("worr", "word");
        });
        Ok(())
    }

    #[bench]
    fn compute_levenshtein_distance_from_crate(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        b.iter(|| {
            _ = super::levenshtein("worr", "word");
        });
        Ok(())
    }
}
