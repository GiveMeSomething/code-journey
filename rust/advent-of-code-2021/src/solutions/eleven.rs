use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_octopus_from_file() -> [[usize; 10]; 10] {
    let file = File::open("src/inputs/eleven.txt").expect("Cannot find/open input file for day 11");
    let reader = BufReader::new(file);

    let mut result: [[usize; 10]; 10] = [[0; 10]; 10];

    let mut i = 0;
    for line in reader.lines() {
        let line = line.expect("Cannot read line");

        let mut j = 0;
        for char in line.chars() {
            result[i][j] = String::from(char).parse::<usize>().unwrap();
            j += 1;
        }
        i += 1;
    }

    return result;
}

pub fn simulate_flash(octopus: &mut [[usize; 10]; 10], step: usize) -> usize {
    let mut flash_count = 0;
    for _ in 0..step {
        simulate_step(octopus);
        flash_count += count_and_finalize_step(octopus);
        println!("{:#?}", octopus);
    }
    return flash_count;
}

fn simulate_step(octopus: &mut [[usize; 10]; 10]) {
    for i in 0..10 {
        for j in 0..10 {
            octopus[i][j] += 1;

            if octopus[i][j] == 10 {
                increase_adjacent(octopus, i, j);
            }
        }
    }
}

fn count_and_finalize_step(octopus: &mut [[usize; 10]; 10]) -> usize {
    let mut flash_count = 0;
    for i in 0..10 {
        for j in 0..10 {
            if octopus[i][j] > 9 {
                flash_count += 1;
                octopus[i][j] = 0;
            }
        }
    }
    return flash_count;
}

fn increase_adjacent(octopus: &mut [[usize; 10]; 10], i: usize, j: usize) {
    let i = i as isize;
    let j = j as isize;
    let positions = vec![
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ];

    for position in positions {
        let (temp_i, temp_j) = position;
        let result = try_increase(octopus, temp_i, temp_j);
        if result == -1 {
            continue;
        }

        if result == 10 {
            // tempi_i and temp_j are guaranteed to be valid usize once passed through try_increase
            let temp_i = usize::try_from(temp_i).unwrap();
            let temp_j = usize::try_from(temp_j).unwrap();
            increase_adjacent(octopus, temp_i, temp_j);
        }
    }
}

// Try to increase and output new value
// -1 indicate failure
fn try_increase(octopus: &mut [[usize; 10]; 10], i: isize, j: isize) -> isize {
    let i = usize::try_from(i).unwrap_or(usize::MAX);
    let j = usize::try_from(j).unwrap_or(usize::MAX);

    if i == usize::MAX || j == usize::MAX {
        return -1;
    }

    if i > 9 || j > 9 {
        return -1;
    }

    // Do NOT increase overflow value
    // This will avoid infinite loop
    if octopus[i][j] > 9 {
        return -1;
    }

    octopus[i][j] += 1;
    return octopus[i][j] as isize;
}
