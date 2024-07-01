use std::io::Error;

use regex::Regex;

#[derive(Debug)]
pub struct Policy {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl TryFrom<String> for Policy {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let regex = Regex::new(r"(\d+)-(\d+) ([a-z]+): (\w+)").expect("Invalid policy regex");

        if let Some(group) = regex.captures(&value) {
            let min: usize = group
                .get(1)
                .expect("No min")
                .as_str()
                .parse()
                .expect("Min value must be a number");
            let max: usize = group
                .get(2)
                .expect("No max")
                .as_str()
                .parse()
                .expect("Max value must be a number");
            let character = group
                .get(3)
                .expect("No policy character")
                .as_str()
                .chars()
                .next()
                .expect("Policy must have a character");
            let password = group.get(4).expect("No password").as_str().to_string();

            return Ok(Policy {
                min,
                max,
                character,
                password,
            });
        }

        Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid policy",
        ))
    }
}

impl Policy {
    pub fn is_valid(&self) -> bool {
        let mut count = 0;

        for character in self.password.chars() {
            if character == self.character {
                count += 1;
            }
        }

        count >= self.min && count <= self.max
    }
}
