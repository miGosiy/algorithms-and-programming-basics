// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem

pub fn breaking_records(scores: Vec<i32>) -> Vec<i32> {
    let mut max_score = scores[0];
    let mut min_score = scores[0];
    let mut max_count = 0;
    let mut min_count = 0;

    for &score in scores.iter().skip(1) {
        if score > max_score {
            max_score = score;
            max_count += 1;
        } else if score < min_score {
            min_score = score;
            min_count += 1;
        }
    }

    vec![max_count, min_count]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(scores), vec![2, 4]);
    }
}