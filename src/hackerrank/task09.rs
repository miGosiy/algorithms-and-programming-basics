// https://www.hackerrank.com/challenges/sock-merchant/problem

use std::collections::HashMap;

pub fn sock_merchant(arr: Vec<i32>) -> i32 {
    let mut count = HashMap::new();
    let mut pairs = 0;

    for x in arr {
        *count.entry(x).or_insert(0) += 1;
    }

    for value in count.values() {
        pairs += value / 2;
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant() {
        let arr = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(arr), 3);
    }
}