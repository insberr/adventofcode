use std::fs;

fn get_file_contnets() -> String {
    return fs::read_to_string("./input.txt").expect("Cannot read the given file");
}

fn part1(con: String) {}

fn part2(con: String) {}

fn main() {
    part1(get_file_contnets());
    part2(get_file_contnets());
}
