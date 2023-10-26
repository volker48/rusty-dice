use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = match args.as_slice() {
        [_, arg] => arg,
        _ => "6",
    };
    let num_words: usize = num.parse().unwrap_or(6);
    let rng = thread_rng();
    let raw_words = include_str!("wordlist.txt");
    let words: Vec<&str> = raw_words.lines().collect();
    let range = Uniform::new(0, words.len());
    let pass = rng
        .sample_iter(range)
        .take(num_words)
        .map(|i| words[i])
        .collect::<Vec<&str>>()
        .join("-");
    println!("{}", pass);
}
