use std::time::SystemTime;
use itertools::Itertools;
use sha256::digest;

fn main() {
    let indexes: [usize; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let chars = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"];
    let words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "a", "b", "c", "d", "e", "f"];
    let mut length = 2;
    let start = SystemTime::now();

    loop {
        for permutation in indexes.iter().permutations(length) {
            let starts: String = permutation.iter().map(|x| chars[**x]).collect();
            let parts = permutation.iter().map(|x| words[**x]).collect_vec();
            let sentence = format!(
                "The SHA256 for this sentence begins with: {} and {}.",
                &parts[0..(parts.len() - 1)].join(", "),
                &parts[parts.len() - 1]
            );
            let digest: String = digest(&sentence);
            if digest.starts_with(&starts) {
                println!("milliseconds: {:?}, {} ", start.elapsed().unwrap().as_millis(), &sentence);
            }
        };
        length += 1;
    }
}
