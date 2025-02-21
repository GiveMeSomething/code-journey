pub fn simplify_path(path: String) -> String {
    let mut char_stack: Vec<char> = path.chars().collect();
    let mut path_stack: Vec<String> = vec![];

    let mut current_substring = "".to_string();
    let mut skip = 0;

    while !char_stack.is_empty() {
        println!("path_stack {:?}", path_stack);

        let current_char = char_stack.pop().unwrap();

        if current_char == '/' {
            if current_substring.is_empty() {
                continue;
            }

            if current_substring == ".." {
                skip += 1;
                current_substring = "".to_string();
                continue;
            }

            if current_substring == "." {
                current_substring = "".to_string();
                continue;
            }

            if skip > 0 {
                skip -= 1;
                current_substring = "".to_string();
                continue;
            }

            path_stack.push(current_substring);
            current_substring = "".to_string();
            continue;
        }

        current_substring = format!("{}{}", current_char, current_substring);
    }

    println!("path_stack {:?} - skip {}", path_stack, skip);

    path_stack.reverse();

    format!("/{}", path_stack.join("/"))
}

#[cfg(test)]
mod test {
    use super::simplify_path;

    #[test]
    fn test_simplify_path() {
        let test_cases: Vec<(&str, &str)> = vec![
            ("/home/", "/home"),
            ("/home//foo/", "/home/foo"),
            ("/home/user/Documents/../Pictures", "/home/user/Pictures"),
            ("/../", "/"),
            ("/../home", "/home"),
            ("/.../a/../b/c/../d/./", "/.../b/d"),
            ("/a/./b/../../c/", "/c"),
        ];

        for (path, expected) in test_cases {
            let result = simplify_path(path.to_string());
            assert_eq!(result, expected);
        }
    }
}
