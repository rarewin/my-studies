#!/usr/bin/env python3

n = int(input())

found = {'1': 1}

def divisors(num):

    ret = num + 1
    s_num = str(num)

    if num == 1:
        return 1

    for i in range(2, n // 2):

        if num % i == 0:
            if s_num in found:
                ret += found[s_num]
                break
            else:
                ret += i

    found[s_num] = ret
    return ret

s = 0

for i in range(n):
    s += divisors(i + 1)

print(s)
