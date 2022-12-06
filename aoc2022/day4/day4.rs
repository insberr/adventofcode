use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Cannot read the given file");
    let assignments = contents.lines();

    let mut total_assignments = 0;

    for assignment in assignments {
        let elfs = assignment.split(",").collect::<Vec<&str>>();
        let elf1 = elfs[0].split("-").collect::<Vec<&str>>();
        let elf2 = elfs[1].split("-").collect::<Vec<&str>>();

        if elf1[0].parse::<i32>().unwrap() >= elf2[0].parse::<i32>().unwrap()
            && elf1[1].parse::<i32>().unwrap() <= elf2[1].parse::<i32>().unwrap()
        {
            // println!("elf 1 {:?} is in {}", elf1, assignment);
            total_assignments += 1;
        } else if elf2[0].parse::<i32>().unwrap() >= elf1[0].parse::<i32>().unwrap()
            && elf2[1].parse::<i32>().unwrap() <= elf1[1].parse::<i32>().unwrap()
        {
            // println!("elf 2 {:?} is in {}", elf1, assignment);
            total_assignments += 1;
        }
    }

    println!("{}", total_assignments);

    // Part 2
    let contents = fs::read_to_string("./input.txt").expect("Cannot read the given file");
    let assignments = contents.lines();

    let mut total_overlapping_assignments = 0;
    let mut overlaps_compare = Vec::new();

    for assignment in assignments {
        let elfs = assignment.split(",").collect::<Vec<&str>>();
        let elf1 = elfs[0].split("-").collect::<Vec<&str>>();
        let elf2 = elfs[1].split("-").collect::<Vec<&str>>();

        let elf1_start = elf1[0].parse::<i32>().unwrap();
        let elf1_end = elf1[1].parse::<i32>().unwrap();
        let elf2_start = elf2[0].parse::<i32>().unwrap();
        let elf2_end = elf2[1].parse::<i32>().unwrap();

        // create a range
        let mut elf1_range = Vec::new();
        for i in elf1_start..elf1_end + 1 {
            elf1_range.push(i);
        }

        let mut elf2_range = Vec::new();
        for i in elf2_start..elf2_end + 1 {
            elf2_range.push(i);
        }

        for i in &elf1_range {
            if elf2_range.contains(&i) {
                total_overlapping_assignments += 1;
                // println!("{} overlaps with {}", assignment, i);
                overlaps_compare.push(assignment);
                break;
            }
        }
    }

    println!("{}", total_overlapping_assignments);

    // let contents = fs::read_to_string("./input.txt").expect("Cannot read the given file");
    // let assignments = contents.lines();
    // for i in assignments {
    //     println!("{} {}", i, overlaps_compare.contains(&i));
    // }
}
