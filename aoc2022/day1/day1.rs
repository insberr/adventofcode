use std::fs;

pub struct Elf {
    id: i32,
    total_calories: i32,
}

pub type Elves = Vec<Elf>;

pub fn part1() -> Elves {
    let contents = fs::read_to_string("./input.txt").expect("Cannot read the given file");

    let mut elves = Elves::new();

    // Split text by empty lines
    let groups = contents.split("\n\n").collect::<Vec<&str>>();

    let mut i = 0;
    for group in groups {
        // println!("Group {}:\n{}", i, group);
        i = i + 1;

        let lines = group.split("\n").collect::<Vec<&str>>();
        // add all the values together
        let total = lines
            .iter()
            .map(|line| line.parse::<i32>().unwrap())
            .sum::<i32>();
        let elf: Elf = Elf {
            id: i,
            total_calories: total,
        };

        elves.push(elf);
    }

    // find the elf with thehighest total_calories
    // sort elves by total_calories
    elves.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));
    println!(
        "Elf {} has the most calories: {}",
        elves[0].id, elves[0].total_calories
    );
    return elves;
}

pub fn part2(sorted_elves: &mut Elves) {
    println!();

    let el1 = sorted_elves[0].total_calories;
    let el2 = sorted_elves[1].total_calories;
    let el3 = sorted_elves[2].total_calories;

    let total_top_3 = el1 + el2 + el3;
    println!("Total calories of top 3 elves: {}", total_top_3);
}

pub fn main() {
    let mut sorted = part1();
    part2(&mut sorted);
}
