use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

pub fn read_report_from_file() -> Vec<Vec<usize>> {
    let input_path = "src/inputs/two.txt";
    let file = File::open(input_path).expect("Failed to open input file");
    let reader = BufReader::new(file);

    let mut reports: Vec<Vec<usize>> = vec![];
    for line in reader.lines() {
        let report = extract_report(line.expect("Failed to read report line"))
            .expect("Failed to extract report");
        reports.push(report);
    }

    reports
}

pub fn extract_report(line: String) -> Result<Vec<usize>, ParseIntError> {
    let parts: Vec<&str> = line.split(" ").collect();
    let mut levels = vec![];

    for part in parts {
        let level = part.parse::<usize>()?;
        levels.push(level);
    }

    Ok(levels)
}

#[cfg(test)]
mod test {
    use super::read_report_from_file;

    #[test]
    fn test_read_report_from_file() {
        let reports = read_report_from_file();
    }
}
