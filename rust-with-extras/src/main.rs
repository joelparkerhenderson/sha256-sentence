use std::sync::OnceLock;
use std::time::SystemTime;
use itertools::Itertools;
//use sha2::{Sha256, Digest};
use sha256::digest;
use rayon::iter::{ParallelIterator, ParallelBridge};

pub mod permutation_with_replacement;

const INDEXES: [usize; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
const CHARS: &'static [&'static str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"];
const WORDS: &'static [&'static str] = &["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "a", "b", "c", "d", "e", "f"];
static START: OnceLock<SystemTime> = OnceLock::new();

fn main() {
    START.set(SystemTime::now()).expect("START");
    let mut length = 2;
    loop {
        let permutations = permutation_with_replacement::iter(&INDEXES, length);
        permutations.par_bridge().for_each(move |x| run(x));
        length += 1;
    }
}

fn run(permutation: Vec<usize>) {
    let starts: String = permutation.iter().map(|x| CHARS[*x]).collect();
    let parts = permutation.iter().map(|x| WORDS[*x]).collect_vec();
    let sentence = format!(
        "The SHA256 for this sentence begins with: {} and {}.",
        &parts[0..(parts.len() - 1)].join(", "),
        &parts[parts.len() - 1]
    );
    let digest: String = digest(&sentence);
    if digest.starts_with(&starts) {
        println!(
            "milliseconds: {:?}, digest: {:?}, starts: {:?}, sentence: {:?}",
            START.get().unwrap().elapsed().unwrap().as_millis(),
            &digest,
            &starts,
            &sentence
        );
    }
}


// // Implementation with sha2 crate
// //let mut hasher = Sha256::new();
// let bytes: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
// loop {
//     for permutation in indexes.iter().permutations(length) {
//         let parts = permutation.iter().map(|x| words[**x]).collect_vec();
//         let sentence = format!(
//             "The SHA256 for this sentence begins with: {} and {}.",
//             &parts[0..(parts.len() - 1)].join(", "),
//             &parts[parts.len() - 1]
//         );
//         let digest = Sha256::digest(&sentence);
//         if digest[0..length] == *permutation {
//             println!("milliseconds: {:?}, {} ", start.elapsed().unwrap().as_millis(), &sentence);
//         }
//     };
//     length += 1;
// }

