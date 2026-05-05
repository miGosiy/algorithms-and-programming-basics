/*
Original solution (Python):

s, t = map(int, input().split())
a, b = map(int, input().split())
m, n = map(int, input().split())

apples = list(map(int, input().split()))
oranges = list(map(int, input().split()))

count_apples = 0
for d in apples:
    if s <= a + d <= t:
        count_apples += 1

count_oranges = 0
for d in oranges:
    if s <= b + d <= t:
        count_oranges += 1

print(count_apples)
print(count_oranges)
*/

pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    let mut count_apples = 0;
    let mut count_oranges = 0;

    for d in apples {
        let pos = a + d;
        if pos >= s && pos <= t {
            count_apples += 1;
        }
    }

    for d in oranges {
        let pos = b + d;
        if pos >= s && pos <= t {
            count_oranges += 1;
        }
    }

    (count_apples, count_oranges)
}