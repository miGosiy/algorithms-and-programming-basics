// https://www.hackerrank.com/challenges/sock-merchant/problem
pub fn diagonal_difference(arr: Vec<Vec<i32>>) -> i32 {
    let n = arr.len();
    let mut main_diagonal = 0;
    let mut secondary_diagonal = 0;

    for i in 0..n {
        main_diagonal += arr[i][i];
        secondary_diagonal += arr[i][n - 1 - i];
    }

    (main_diagonal - secondary_diagonal).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference() {
        let arr = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];

        assert_eq!(diagonal_difference(arr), 15);
    }
}