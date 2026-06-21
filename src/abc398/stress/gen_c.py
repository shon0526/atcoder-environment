"""abc398-c 用のランダム入力生成。

ABC398-C: N 人がそれぞれ整数 A_i を持つ。他の誰とも値が被らない人のうち、
値が最大の人の番号を出力する（いなければ -1）。

argv[1] を seed として random.seed() に渡すので、同じ seed なら同じ入力を再現できる。
愚直解が O(N^2) なので N は小さく、値も小さくして被り（=ユニーク判定）が起きやすくする。

使い方:
    python gen_c.py <seed>
"""

import random
import sys


def main() -> None:
    seed = int(sys.argv[1])
    random.seed(seed)

    # N と値を小さくして、ユニーク要素・重複要素が混在する反例を出やすくする。
    n = random.randint(1, 8)
    values = [random.randint(1, 5) for _ in range(n)]

    print(n)
    print(*values)


if __name__ == "__main__":
    main()
