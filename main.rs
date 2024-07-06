use std::{fs, cmp::Ordering, io};

const TOTAL_WORDS: usize = 14855; // Total possible wordle words
const LETTER_SCORES: [i32; 26] = [7128, 1849, 2246, 2735, 7455, 1240, 1864, 1993, 4381, 342, 1753, 3780, 2414, 3478, 5212, 2436, 145, 4714, 7319, 3707, 2927, 801, 1127, 326, 2400, 503]; // The frequency of each letter in the possible wordle words
static mut WORDS: [[u8; 5]; TOTAL_WORDS] = [[0u8; 5]; TOTAL_WORDS]; // The array of all possible wordle words

// This function loads all possible worlde words into the WORDS array
fn load_words(){
    let words_bytes = fs::read("words.txt").expect("Could not find \"words.txt\"");
    for i in 0..TOTAL_WORDS {
        unsafe { WORDS[i] = [
            words_bytes[7*i+0],
            words_bytes[7*i+1],
            words_bytes[7*i+2],
            words_bytes[7*i+3],
            words_bytes[7*i+4],
        ];}
    }
}

// This function return a value on how common a word is, the higher the better
// If a letter is repeated in the word, it counts for less
fn word_score(word: &[u8; 5]) -> i32 {
    let mut score: i32 = 0;
    for letter in word {
        score += LETTER_SCORES[(letter - 97) as usize]/(word.iter().filter(|&n| *n == *letter).count() as i32);
    }
    return score;
}

// This finds the "best word" in an array of words according to the word_score function
fn best_word(words: &Vec<[u8; 5]>) -> [u8; 5] {
    let Some(guess) = 
        words.iter()
            .max_by(|a, b| match (word_score(a)-word_score(b)).signum() {
                1  => Ordering::Greater,
                0  => Ordering::Equal,
                -1 => Ordering::Less,
                _  => Ordering::Equal,
            })
        else { panic!(); };
    return *guess;
}

// Converts the ascii representation of a word to a string
fn word_to_string(word: &[u8; 5]) -> String {
    return String::from(std::str::from_utf8(word).expect("invalid utf-8 sequence"));
}

// Represents the constraints placed on the word, (character, position)
enum Filter {
    Green(u8, usize),
    Yellow(u8, usize),
    Black(u8),
}

fn filter_words(mut words: Vec<[u8; 5]>, filters: &Vec<Filter>) -> Vec<[u8; 5]>{
    for filter in filters {
        match filter {
            Filter::Green(letter, pos) => {
                words = words.into_iter()
                    .filter(|w| w[*pos] == *letter)
                    .collect();
            },
            Filter::Yellow(letter, pos) => {
                words = words.into_iter()
                    .filter(|w| w.contains(letter) && w[*pos] != *letter)
                    .collect();
            },
            Filter::Black(letter) => {
                words = words.into_iter()
                    .filter(|w| !w.contains(letter))
                    .collect();
            },
        }
    }
    return words;
}

fn prompt_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    return input.trim().to_string();
}

fn collect_input() -> Vec<Filter> {
    println!("Input the word that you entered, and after its color result (i.e. gyyby)");
    let guess = prompt_input("Enter your word: ");
    let color = prompt_input("Enter the colors: ");

    let mut filters: Vec<Filter> = vec![];
    for i in 0..5 {
        filters.push (match color.chars().nth(i).unwrap() {
            'g' => Filter::Green(guess.chars().nth(i).unwrap() as u8, i),
            'y' => Filter::Yellow(guess.chars().nth(i).unwrap() as u8, i),
            'b' => Filter::Black(guess.chars().nth(i).unwrap() as u8),
            _   => Filter::Black(0),
        })
    }

    return filters;
}

fn main() {
    load_words();
    let mut vec_words: Vec<[u8; 5]> = unsafe { WORDS.to_vec() };
    println!("Welcome to Wordle Solver!");
    println!("(We recommend that you start your game with 'adieu')");
    while vec_words.len() > 1 {
        vec_words = filter_words(vec_words, &collect_input());
        println!("We suggest: {}", word_to_string(&best_word(&vec_words)));
    }
}