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

fn compute_score(v: Vec<char>, arr: &[u8]) -> u8 {
    let mut i: usize = 0;
    let mut score: u8 = 0;
    let k: usize = v.len() - 1;
    while i < k {
        match v[i] {
            'A' | 'a' => {
                score += arr[0];
            }
            'B' | 'b' => {
                score += arr[1];
            }
            'C' | 'c' => {
                score += arr[2];
            }
            'D' | 'd' => {
                score += arr[3];
            }
            'E' | 'e' => {
                score += arr[4];
            }
            'F' | 'f' => {
                score += arr[5];
            }
            'G' | 'g' => {
                score += arr[6];
            }
            'H' | 'h' => {
                score += arr[7];
            }
            'I' | 'i' => {
                score += arr[8];
            }
            'J' | 'j' => {
                score += arr[9];
            }
            'K' | 'k' => {
                score += arr[10];
            }
            'L' | 'l' => {
                score += arr[11];
            }
            'M' | 'm' => {
                score += arr[12];
            }
            'N' | 'n' => {
                score += arr[13];
            }
            'O' | 'o' => {
                score += arr[14];
            }
            'P' | 'p' => {
                score += arr[15];
            }
            'Q' | 'q' => {
                score += arr[16];
            }
            'R' | 'r' => {
                score += arr[17];
            }
            'S' | 's' => {
                score += arr[18];
            }
            'T' | 't' => {
                score += arr[19];
            }
            'U' | 'u' => {
                score += arr[20];
            }
            'V' | 'v' => {
                score += arr[21];
            }
            'W' | 'w' => {
                score += arr[22];
            }
            'X' | 'x' => {
                score += arr[23];
            }
            'Y' | 'y' => {
                score += arr[24];
            }
            'Z' | 'z' => {
                score += arr[25];
            }
            _ => score += 0,
        }
        i += 1;
    }
    return score;
}
fn compute_winner(score1: u8, score2: u8) {
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
fn main() {
    let arr: [u8; 26] = [
        1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10,
    ];

    let v1: Vec<char> = input(1);
    let v2: Vec<char> = input(2);
    let score1 = compute_score(v1, &arr);
    let score2 = compute_score(v2, &arr);
    compute_winner(score1, score2);
}
