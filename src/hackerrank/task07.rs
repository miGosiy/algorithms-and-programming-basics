// https://www.hackerrank.com/challenges/between-two-sets/problem
/*
n = int(input())
scores = list(map(int, input().split()))

max_score = scores[0]
min_score = scores[0]
max_count = 0
min_count = 0

for score in scores[1:]:
    if score > max_score:
        max_score = score
        max_count += 1
    elif score < min_score:
        min_score = score
        min_count += 1

print(max_count, min_count)
*/