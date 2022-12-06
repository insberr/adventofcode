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

    // Part 2

    let contents = fs::read_to_string("./input.txt").expect("Cannot read the given file");
    let backpacks = contents.lines();
    let mut groups: Vec<Vec<&str>> = Vec::new();
    let mut ii = 0; // do 3
    let mut i = 0; // full index
    let mut group = 0;
    groups.push(Vec::new());
    for pack in backpacks {
        if ii == 3 {
            ii = 0;
            groups.push(Vec::new());
            group = group + 1;
        }
        groups[group].push(pack);
        i = i + 1;
        ii = ii + 1;
    }

    let mut total_part2 = 0;
    for group in groups {
        let mut shared_1_2 = String::new();
        let mut shared_2_3 = String::new();

        for letter in group[0].chars() {
            if group[1].contains(letter) {
                shared_1_2.push(letter);
            }
            if group[2].contains(letter) {
                shared_2_3.push(letter);
            }
        }
        for letter in shared_1_2.chars() {
            if shared_2_3.contains(letter) {
                // Kinda stolen from wackery
                if letter.is_uppercase() {
                    total_part2 = total_part2
                        + (26 + letter.to_lowercase().to_string().chars().nth(0).unwrap() as i32
                            - 96);
                } else {
                    total_part2 = total_part2 + letter as i32 - 96;
                }
                break;
                // println!("{}", letter as u8);
            }
        }
    }

    println!("{}", total_part2);
}
