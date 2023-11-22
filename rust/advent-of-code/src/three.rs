use std::{io::{Lines, BufReader, BufRead}, fs::File}

fn read_rucksack_from_file() -> Lines<BufReader<File>> {
  let file = match File::open("src/inputs/three.txt") {
      Ok(file) => file,
      Err(error) => panic!("Cannot open day 3 input with error {:?}", error),
  };

  let reader = BufReader::new(file);
  return reader.lines();
}

