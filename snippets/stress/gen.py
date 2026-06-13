"""ランダム入力生成の雛形。

各問題用に gen_<problem>.py という名前にコピーして、その問題の制約・入出力形式に
合わせて中身を実装する。argv[1] を seed として random.seed() に渡すので、同じ seed
なら同じ入力を再現できる。反例を見つけやすいよう N 等は小さめ（例: 1〜8）にする。

使い方:
    python gen_<problem>.py <seed>
"""

import random
import sys


def main() -> None:
    seed = int(sys.argv[1])
    random.seed(seed)

    # TODO: 問題の制約に合わせてランダム入力を生成する。
    # 以下は「N と長さ N の配列」のサンプル。実際の入出力形式に置き換えること。
    n = random.randint(1, 8)
    values = [random.randint(1, 20) for _ in range(n)]

    print(n)
    print(*values)


if __name__ == "__main__":
    main()
