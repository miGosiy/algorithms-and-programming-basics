/*
Original solution (Python):

n, m = map(int, input().split())
a = list(map(int, input().split()))
b = list(map(int, input().split()))

count = 0
for x in range(max(a), min(b) + 1):
    ok = True
    for i in a:
        if x % i != 0:
            ok = False
            break
    for j in b:
        if j % x != 0:
            ok = False
            break
    
    if ok:
        count += 1

print(count)
*/

pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;

    let start = *a.iter().max().unwrap();
    let end = *b.iter().min().unwrap();

    for x in start..=end {
        let mut ok = true;

        for &i in a {
            if x % i != 0 {
                ok = false;
                break;
            }
        }

        for &j in b {
            if j % x != 0 {
                ok = false;
                break;
            }
        }

        if ok {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let a = [2, 4];
        let b = [16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }
}