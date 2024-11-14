use std::collections::{BinaryHeap, HashMap};

pub struct CharCount {
    value: char,
    count: usize,
}

impl PartialEq for CharCount {
    fn eq(&self, other: &Self) -> bool {
        self.count.eq(&other.count)
    }
}

impl Eq for CharCount {}

impl PartialOrd for CharCount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.count.partial_cmp(&other.count)
    }
}

impl Ord for CharCount {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

pub fn frequency_sort(s: String) -> String {
    let mut char_count: HashMap<char, usize> = HashMap::new();
    for char in s.chars().into_iter() {
        let current_count = match char_count.get(&char) {
            Some(value) => *value,
            None => 0,
        };

        char_count.insert(char, current_count + 1);
    }

    let mut max_heap: BinaryHeap<CharCount> = BinaryHeap::new();
    for (character, count) in char_count {
        max_heap.push(CharCount {
            value: character,
            count,
        });
    }

    let mut result: String = String::from("");
    while let Some(node) = max_heap.pop() {
        result = format!("{}{}", result, String::from(node.value).repeat(node.count))
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_frequency_sort() {
        let test_cases: Vec<(String, Vec<String>)> = vec![
            ("tree".to_string(), vec!["eert".to_string()]),
            (
                "cccaaa".to_string(),
                vec!["cccaaa".to_string(), "aaaccc".to_string()],
            ),
            (
                "Aabb".to_string(),
                vec!["bbAa".to_string(), "bbaA".to_string()],
            ),
        ];

        for (input, expected) in test_cases {
            let actual = frequency_sort(input);
            assert!(expected.contains(&actual));
        }
    }
}
