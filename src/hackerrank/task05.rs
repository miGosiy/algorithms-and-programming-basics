// https://www.hackerrank.com/challenges/apple-and-orange/problem
pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        return "NO".to_string();
    }

    if (x2 - x1) % (v1 - v2) == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yes() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_no() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }
}