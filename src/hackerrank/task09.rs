// https://www.hackerrank.com/challenges/migratory-birds/problem
/*
n = int(input())
arr = list(map(int, input().split()))

count = {}
pairs = 0

for x in arr:
    if x in count:
        count[x] += 1
    else:
        count[x] = 1

for value in count.values():
    pairs += value // 2

print(pairs)
*/