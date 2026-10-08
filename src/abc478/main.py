import random as rnd

n = 20000
k = rnd.randint(1, n)
print(f"{n} {k}")

a = [rnd.randint(1, n) for _ in range(n)]
print(" ".join(map(str, a)))
