import random as rnd

n = 2 * (10**5)
m = 2 * (10**6)
a = [rnd.randint(1, 10**9) for _ in range(n)]

print(f"{n} {m}")
print(*a)
