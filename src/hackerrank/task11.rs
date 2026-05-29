// https://www.hackerrank.com/challenges/drawing-book/problem
pub fn page_count(n: i32, p: i32) -> i32 {
    let front = p / 2;
    let back = n / 2 - p / 2;

    front.min(back)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_count() {
        assert_eq!(page_count(6, 2), 1);
        assert_eq!(page_count(5, 4), 0);
        assert_eq!(page_count(5, 3), 1);
    }
}