use std::io;
use std::io::Write;

fn input(n: u8) -> Vec<char> {
    print!("Enter word for person {}: ", n);

    io::stdout().flush().unwrap(); //Just makes print! to work
    let mut input_line = String::new(); //New string used for input
    io::stdin() // the rough equivalent of `std::cin`
        .read_line(&mut input_line) // actually read the line
        .expect("Failed to read line"); // which can fail, however

    let chars: Vec<char> = input_line.chars().collect(); //Convert string to char array
    return chars;
}

fn compute_winner(score1: u16, score2: u16) {
    if score1 > score2 {
        println!(
            "Person 1 won with {} points against person 2 with {} points.",
            score1, score2
        )
    }
    if score1 < score2 {
        println!(
            "Person 2 won with {} points against person 1 with {} points.",
            score2, score1
        )
    }
    if score1 == score2 {
        println!("Tie. Person 1 points equals person two points {}", score1);
    }
}

fn compute(v: Vec<char>, points: &[u16], small_letters: &[char], capital_letters: &[char]) -> u16 {
    //let mut i: usize = 0;
    // let mut k: usize = 0;
    let mut score = 0;
    for n in 0..=v.len() - 1 {
        //while k < v.len() - 1 {
        if v[n].is_uppercase() {
            for k in 0..=25 {
                if v[n] == capital_letters[k] {
                    score += points[k];
                    break;
                }
            }
        }
        else if v[n].is_lowercase() {
            for k in 0..=25 {
                if v[n] == small_letters[k] {
                    score += points[k];
                    break;
                }
            }
        } 
        else {
        }
    }
    return score;
}

fn main() {
    let points: [u16; 26] = [
        1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10,
    ];
    let small_letters: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    // Ascii Values for capital letters of alphabets
    let capital_letters: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let v1: Vec<char> = input(1);
    let v2: Vec<char> = input(2);
    let score1 = compute(v1, &points, &small_letters, &capital_letters);
    let score2 = compute(v2, &points, &small_letters, &capital_letters);
    compute_winner(score1, score2);
    return;
}
