// https://www.hackerrank.com/challenges/apple-and-orange/problem
/*
Original solution (Python):

x1, v1, x2, v2 = map(int, input().split())
if v1 <= v2:
    print("NO")
else:
    if (x2 - x1) % (v1 - v2) == 0:
        print("YES")
    else:
        print("NO")
*/

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