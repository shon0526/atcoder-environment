import random as rd

n = 20
print(n)

ks = [rd.randint(1, 100000000) for _ in range(n)]

print(" ".join(map(str, ks)))
