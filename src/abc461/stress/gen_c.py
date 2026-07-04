"""abc461 C 用のランダム入力生成。

使い方:
    python3 gen_c.py <seed>

argv[1] を seed として random.seed() に渡すので、同じ seed なら同じ入力を再現できる。
制約 (1 <= M <= K <= N、色は 1..N、少なくとも M 種類の色が存在) を満たす小さめの入力を出す。
"""

import random
import sys


def main() -> None:
    seed = int(sys.argv[1])
    random.seed(seed)

    n = random.randint(1, 10001)
    l = random.randint(2, 10001)
    print(f"{n} {l}")


if __name__ == "__main__":
    main()
