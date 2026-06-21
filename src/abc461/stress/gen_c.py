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

    n = random.randint(1, 8)
    k = random.randint(1, n)

    # 各宝石の色を 1..N からランダムに割り当てる。
    colors = [random.randint(1, n) for _ in range(n)]
    distinct = len(set(colors))

    # M は K 以下かつ「実際に存在する色の種類数」以下にする。
    m = random.randint(1, min(k, distinct))

    lines = [f"{n} {k} {m}"]
    for c in colors:
        v = random.randint(1, 20)
        lines.append(f"{c} {v}")

    print("\n".join(lines))


if __name__ == "__main__":
    main()
