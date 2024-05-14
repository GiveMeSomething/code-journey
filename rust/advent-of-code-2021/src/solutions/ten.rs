use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_code_from_file() -> Vec<String> {
    let file = File::open("src/inputs/ten.txt").expect("Cannot find input file for day 10");
    let reader = BufReader::new(file);

    let mut code: Vec<String> = vec![];
    for line in reader.lines() {
        let current_line = line.expect("Cannot read line");
        code.push(current_line);
    }

    return code;
}

pub fn sum_illegal_point(lines: &Vec<String>) -> isize {
    let mut sum = 0;
    for line in lines {
        sum += get_line_illegal_point(line.as_str());
    }
    return sum;
}

fn get_line_illegal_point(line: &str) -> isize {
    let mut queue: Vec<char> = vec![];
    for token in line.chars() {
        if token == '(' || token == '[' || token == '{' || token == '<' {
            queue.push(token);
            continue;
        }

        let valid = match token {
            ')' => queue[queue.len() - 1] == '(',
            ']' => queue[queue.len() - 1] == '[',
            '}' => queue[queue.len() - 1] == '{',
            '>' => queue[queue.len() - 1] == '<',
            _ => true,
        };
        if !valid {
            return get_char_illegal_point(token);
        }

        queue.pop();
    }

    return 0;
}

fn get_char_illegal_point(token: char) -> isize {
    match token {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use super::sum_illegal_point;

    #[test]
    fn test_calculate_illegal_point() {
        let inputs: Vec<&str> = vec![
            "[<(<(<(<{}))><([]([]()",
            "[{[{({}]{}}([{[{{{}}([]",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "<{([([[(<>()){}]>(<<{{",
        ];
        let expected: Vec<isize> = vec![3, 57, 1197, 25137];

        for i in 0..inputs.len() {
            let current_input = vec![String::from(inputs[i])];
            let current_expected = expected[i];

            let result = sum_illegal_point(&current_input);
            assert_eq!(
                result,
                expected[i],
                "Expect {} but receive {} for case {:?}",
                current_expected.to_string(),
                result,
                current_input
            );
        }
    }
}
