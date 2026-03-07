pub fn staircase(n: usize) -> Vec<String> {
    let mut result = Vec::new();

    for i in 1..=n {
        let spaces = " ".repeat(n - i);
        let hashes = "#".repeat(i);
        result.push(format!("{}{}", spaces, hashes));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase() {
        let result = staircase(4);

        let expected = vec![
            "   #".to_string(),
            "  ##".to_string(),
            " ###".to_string(),
            "####".to_string(),
        ];

        assert_eq!(result, expected);
    }
}
