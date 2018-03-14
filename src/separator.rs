#![allow(unused_variables)]
#![allow(dead_code)]

pub fn separate(s: &str, sep: char) -> String {
    let number_parts = s.split('.').collect::<Vec<&str>>();
    let integer = number_parts.first().unwrap();
    let remainder = number_parts.last().unwrap();
    let mut result = String::with_capacity(integer.len() + ((integer.len() - 1) / 3));
    let mut i = integer.len();

    for c in integer.chars() {
        result.push(c);
        i -= 1;
        if i > 0 && i % 3 == 0 {
            result.push(sep);
        }
    }

    if number_parts.len() > 1 {
        format!("{}.{}", result, remainder)
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::separate;

    #[test]
    fn test_separator() {
        assert_eq!("100".to_string(), separate("100", ','));
        assert_eq!("1,000".to_string(), separate("1000", ','));
        assert_eq!("1,000,000".to_string(), separate("1000000", ','));
        assert_eq!("1,000,000.12".to_string(), separate("1000000.12", ','));
        assert_eq!("1,000,000.2222".to_string(), separate("1000000.2222", ','));
    }
}
