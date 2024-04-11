use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_location_from_file() -> Vec<Vec<usize>> {
    let file = File::open("src/inputs/nine.txt").expect("Cannot find/open input file for day 9");
    let reader = BufReader::new(file);

    let mut result: Vec<Vec<usize>> = vec![];
    for line in reader.lines() {
        let current_line = line.unwrap();

        result.push(
            current_line
                .split("")
                .filter_map(|part| part.parse().ok())
                .collect(),
        );
    }

    return result;
}

pub fn sum_risk_level(locations: &Vec<Vec<usize>>) -> usize {
    let mut sum = 0;

    let row = locations.len();
    let col = locations[0].len();

    for i in 0..row {
        for j in 0..col {
            sum += if is_low_location(locations, i, j) {
                locations[i][j] + 1
            } else {
                0
            };
        }
    }

    return sum;
}

fn is_low_location(locations: &Vec<Vec<usize>>, i: usize, j: usize) -> bool {
    let current = locations[i][j];
    let limit_i = locations.len() - 1;
    let limit_j = locations[0].len() - 1;

    let low_top = if i == 0 {
        true
    } else {
        current < locations[i - 1][j]
    };
    let low_bot = if i == limit_i {
        true
    } else {
        current < locations[i + 1][j]
    };
    let low_left = if j == 0 {
        true
    } else {
        current < locations[i][j - 1]
    };
    let low_right = if j == limit_j {
        true
    } else {
        current < locations[i][j + 1]
    };

    if low_top && low_bot && low_left && low_right {
        return true;
    }
    return false;
}

// Function to add a outer layer of MAX usize
// pub fn prevent_out_of_bound(locations: &mut Vec<Vec<usize>>) {
//     // Append an outer layer to avoid index out of bound
//     let rows = locations.len();
//     let cols = locations[0].len();

//     for row in 0..rows {
//         locations[row].insert(0, usize::MAX);
//         locations[row].push(usize::MAX);
//     }

//     locations.insert(0, vec![usize::MAX; cols + 2]);
//     locations.push(vec![usize::MAX; cols + 2]);
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_risk_level() {
        let inputs = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        let expected = 15;
        let receive = sum_risk_level(&inputs);

        assert_eq!(receive, expected);
    }
}
