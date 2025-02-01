use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;
use std::error::Error;
use std::{cmp::Ordering, io::Read};
use std::fs::File;
use clap::{crate_authors, crate_version, value_parser, Arg, ArgMatches, Command};

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



fn get_words_from_file(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut f = File::open(file_path)?;
    let buf = &mut Default::default();
    let _ = f.read_to_string(buf)?;
    let words_ref = buf.split_word_bounds().collect::<Vec<&str>>();

    Ok(words_ref.iter().map(|word_ref| word_ref.to_string()).collect())
}

fn perform_spellcheck() {

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

enum Spelling {
    Correct,
    Incorrect(String, String, u32)
}

fn check_word_against_dictionary(word: &String, dictionary: &Vec<String>) -> Result<Spelling, Box<dyn std::error::Error>> {
    let mut min_distance: u32 = u32::MAX;
    let closest_word: String = Default::default();
    for dict_entry in dictionary {
        let distance = compute_levenshtein_distance(&word, &dict_entry);
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
    // let buf: &mut String = &mut Default::default();
    // let s = "a̐éö̲\r\n";
    // let a = s.split_word_bounds().collect::<Vec<&str>>();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn throws_error_on_parse_args_when_no_args_provided() {

    }
}
