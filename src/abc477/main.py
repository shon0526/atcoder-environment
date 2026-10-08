import random as rnd

n = 3
q = 10
print(f"{n} {q}")

for i in range(10):
    num = rnd.randint(1, 2)
    if num == 1:
        c = rnd.randint(1, 3)
    else:
        c = rnd.randint(1, 9)
    print(f"{num} {c}")
