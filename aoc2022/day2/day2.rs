use std::fs;

pub fn main() {
    println!("Day 2");

    let contents = fs::read_to_string("./input.txt").expect("Cannot read the given file");
    let mut plays = contents.split("\n").collect::<Vec<&str>>();

    let mut total_score = 0;
    for play in plays {
        let mut opponent = play.split(" ").collect::<Vec<&str>>()[0];
        let mut me = play.split(" ").collect::<Vec<&str>>()[1];

        // A and X rock
        // B and Y paper
        // C and Z scissors

        if me == "X" {
            total_score = total_score + 1;
        } else if me == "Y" {
            total_score = total_score + 2;
        } else if me == "Z" {
            total_score = total_score + 3;
        }

        if opponent == "A" {
            if me == "X" {
                total_score = total_score + 3;
            } else if me == "Y" {
                total_score = total_score + 6;
            } else if me == "Z" {
                total_score = total_score + 0;
            }
        } else if opponent == "B" {
            if me == "X" {
                total_score = total_score + 0;
            } else if me == "Y" {
                total_score = total_score + 3;
            } else if me == "Z" {
                total_score = total_score + 6;
            }
        } else if opponent == "C" {
            if me == "X" {
                total_score = total_score + 6;
            } else if me == "Y" {
                total_score = total_score + 0;
            } else if me == "Z" {
                total_score = total_score + 3;
            }
        }
        // println!("{} {}", play, total_score);
    }

    println!("Total score: {}", total_score);
    part2(contents);
}

fn part2(mut con: String) {
    let mut plays = con.split("\n").collect::<Vec<&str>>();

    let mut total_score = 0;
    for play in plays {
        let mut opponent = play.split(" ").collect::<Vec<&str>>()[0];
        let mut me = play.split(" ").collect::<Vec<&str>>()[1];

        // A rock 1
        // B paper 2
        // C scissors 3

        // X lose
        // Y draw
        // Z win

        if me == "X" {
            total_score = total_score + 0;

            if opponent == "A" {
                total_score = total_score + 3;
            } else if opponent == "B" {
                total_score = total_score + 1;
            } else if opponent == "C" {
                total_score = total_score + 2;
            }
        } else if me == "Y" {
            total_score = total_score + 3;
            if opponent == "A" {
                total_score = total_score + 1;
            } else if opponent == "B" {
                total_score = total_score + 2;
            } else if opponent == "C" {
                total_score = total_score + 3;
            }
        } else if me == "Z" {
            total_score = total_score + 6;

            if opponent == "A" {
                total_score = total_score + 2;
            } else if opponent == "B" {
                total_score = total_score + 3;
            } else if opponent == "C" {
                total_score = total_score + 1;
            }
        }
    }
    println!("Total score: {}", total_score);
}
