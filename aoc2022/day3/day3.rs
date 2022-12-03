use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Cannot read the given file");
    let backpacks = contents.lines();
    let mut total = 0;
    for pack in backpacks {
        // q: how do you split a string in half

        let compartments = pack.split_at(pack.len() / 2);

        // find shared letters
        let mut shared: char = 'a';
        for letter in compartments.0.chars() {
            if compartments.1.contains(letter) {
                shared = letter;
                break;
            }
        }

        // where is the letter located in the alphabet
        let mut index = 0;
        for letter in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            if letter == shared {
                break;
            }
            index += 1;
        }
        total += index + 1;
    }
    println!("{}", total);
}
