use unicode_segmentation::UnicodeSegmentation;
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
    Ok(args.get_many::<String>("files")
        .unwrap() // todo replace this
        .map(|s| s.to_string()).collect())
}



fn get_words_from_file(file_path: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

fn check_word_against_dictionary(word: String) -> Result<u32, Box<dyn std::error::Error>> {
    let dictionary = get_words_from_file("training.txt".to_string())?;
    let mut min: u32 = u32::MAX;
    for dict_entry in dictionary {

        let tmp = compute_levenshtein_distance(&word, &dict_entry);
        if tmp < min {
            min = tmp
        }

    }
    Ok(min)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let distance = compute_levenshtein_distance("based", "baser");
    println!("{:#?}", distance);
    let files = parse_args()?;
    let mut vals: Vec<u32> = Default::default();
    for file in files {
        let words = get_words_from_file(file)?;
        for word in words {
            let tmp = check_word_against_dictionary(word)?;
            if tmp != 0 {
                vals.push(tmp);
            }
        }
        
    }
    // let buf: &mut String = &mut Default::default();
    // let s = "a̐éö̲\r\n";
    // let a = s.split_word_bounds().collect::<Vec<&str>>();
    Ok(())
}
