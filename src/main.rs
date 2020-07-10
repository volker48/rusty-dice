use rand::{thread_rng, Rng};
use rand::distributions::Uniform;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_words: usize = args[1].parse().unwrap();
    let mut rng = thread_rng();
    let raw_words = include_str!("wordlist.txt");
    let words: Vec<&str> = raw_words.lines().collect();
    let range = Uniform::new(0, words.len());
    for index in (&mut rng).sample_iter(range).take(num_words){
        println!("word {}", words[index]);
    }
}
