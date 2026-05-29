// https://www.hackerrank.com/challenges/migratory-birds/problem

pub fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut count = vec![0; 6];

    for x in arr {
        count[x as usize] += 1;
    }

    let max_count = *count.iter().max().unwrap();

    for i in 1..=5 {
        if count[i] == max_count {
            return i as i32;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(arr), 4);
    }
}