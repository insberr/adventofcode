use std::fs;

fn get_file_contnets() -> String {
    return fs::read_to_string("./input.txt").expect("Cannot read the given file");
}
fn part_1(con: String) {
    let seperated = con.split("\n\n").collect::<Vec<&str>>();
    let mut input_crates = seperated[0].split("\n").collect::<Vec<&str>>();
    let input_instructions = seperated[1].split("\n").collect::<Vec<&str>>();

    let mut stacks: Vec<Vec<&str>> = Vec::new();

    input_crates.pop();
    input_crates.reverse();

    for level in input_crates {
        let crates = level.split(" ").collect::<Vec<&str>>();

        let mut blank_crates = 0;
        let mut new_crates = Vec::new();
        for c in crates {
            if c == "" {
                blank_crates += 1;
                if blank_crates == 4 {
                    new_crates.push("");
                    blank_crates = 0;
                }
                continue;
            }
            new_crates.push(c);
        }

        // println!("{:?}", new_crates);

        for (i, cr) in new_crates.iter().enumerate() {
            if stacks.len() <= i {
                stacks.push(Vec::new());
            }
            if *cr == "" {
                continue;
            }
            stacks[i].push(cr);
        }
    }

    // println!("{:?}", stacks);

    // Instructions
    for instruction in input_instructions {
        let amount = instruction.split(" ").collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();
        let start_stack = instruction.split(" ").collect::<Vec<&str>>()[3]
            .parse::<usize>()
            .unwrap()
            - 1;
        let move_to_stack = instruction.split(" ").collect::<Vec<&str>>()[5]
            .parse::<usize>()
            .unwrap()
            - 1;

        let stack_len = stacks[start_stack].len();
        let mut selected_crates = stacks[start_stack].split_off(stack_len - amount);
        selected_crates.reverse();
        stacks[move_to_stack].append(&mut selected_crates);
    }

    let mut top_letters = Vec::new();

    for stack in stacks {
        top_letters.push(stack[stack.len() - 1]);
    }

    println!(
        "{:?}",
        top_letters.join("").replace("[", "").replace("]", "")
    );

    return;
}

fn part_2(con: String) {
    let seperated = con.split("\n\n").collect::<Vec<&str>>();
    let mut input_crates = seperated[0].split("\n").collect::<Vec<&str>>();
    let input_instructions = seperated[1].split("\n").collect::<Vec<&str>>();

    let mut stacks: Vec<Vec<&str>> = Vec::new();

    input_crates.pop();
    input_crates.reverse();

    for level in input_crates {
        let crates = level.split(" ").collect::<Vec<&str>>();

        let mut blank_crates = 0;
        let mut new_crates = Vec::new();
        for c in crates {
            if c == "" {
                blank_crates += 1;
                if blank_crates == 4 {
                    new_crates.push("");
                    blank_crates = 0;
                }
                continue;
            }
            new_crates.push(c);
        }

        // println!("{:?}", new_crates);

        for (i, cr) in new_crates.iter().enumerate() {
            if stacks.len() <= i {
                stacks.push(Vec::new());
            }
            if *cr == "" {
                continue;
            }
            stacks[i].push(cr);
        }
    }

    // println!("{:?}", stacks);

    // Instructions
    for instruction in input_instructions {
        let amount = instruction.split(" ").collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();
        let start_stack = instruction.split(" ").collect::<Vec<&str>>()[3]
            .parse::<usize>()
            .unwrap()
            - 1;
        let move_to_stack = instruction.split(" ").collect::<Vec<&str>>()[5]
            .parse::<usize>()
            .unwrap()
            - 1;

        let stack_len = stacks[start_stack].len();
        let mut selected_crates = stacks[start_stack].split_off(stack_len - amount);
        // the only difference between part 1 and 2 is this line : )
        // selected_crates.reverse();
        stacks[move_to_stack].append(&mut selected_crates);
    }

    let mut top_letters = Vec::new();

    for stack in stacks {
        top_letters.push(stack[stack.len() - 1]);
    }

    println!(
        "{:?}",
        top_letters.join("").replace("[", "").replace("]", "")
    );

    return;
}

fn main() {
    part_1(get_file_contnets());
    part_2(get_file_contnets());
}
