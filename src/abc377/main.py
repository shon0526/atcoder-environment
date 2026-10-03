import random as rnd

n = 1000000000
m = 200000

print(f"{n} {m}")

a = [rnd.randint(1, 1000000000) for _ in range(m)]
b = [rnd.randint(1, 1000000000) for _ in range(m)]

for i in range(m):
    print(f"{a[i]} {b[i]}")
