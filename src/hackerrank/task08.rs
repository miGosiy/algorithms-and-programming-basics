// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem
/*
n = int(input())
arr = list(map(int, input().split()))
count = [0] * 6

for x in arr:
    count[x] += 1

max_count = max(count)

for i in range(1, 6):
    if count[i] == max_count:
        print(i)
        break
*/