import random as rnd

n = 2000000000
m = 200000
x = [rnd.randint(1, n) for _ in range(m)]
y = [rnd.randint(1, 2000000000) for _ in range(m)]

print(f"{n} {m}")
print(" ".join(map(str, x)))
print(" ".join(map(str, y)))
