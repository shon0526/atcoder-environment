use cargo_snippet::snippet;

#[snippet(name = "math")]
pub mod math {
    // nを素因数分解し、(素因数, 指数)の昇順ベクタを返す（O(√n)）
    pub fn prime_fact(n: usize) -> Vec<(usize, usize)> {
        if n < 4 {
            return vec![(n, 1)];
        }

        let mut n = n;
        let mut fact = Vec::new();

        for i in 2..=n.isqrt() {
            if n % i == 0 {
                let mut cnt = 0;
                while n % i == 0 {
                    cnt += 1;
                    n /= i;
                }
                fact.push((i, cnt));
            }
        }
        if n != 1 {
            fact.push((n, 1));
        }
        fact
    }
}

#[cfg(test)]
mod tests {
    use super::math;

    #[test]
    fn prime_fact_works() {
        let n1 = 6;
        let n2 = 18;

        let fact1 = math::prime_fact(n1);
        let fact2 = math::prime_fact(n2);

        let ans1 = vec![(2, 1), (3, 1)];
        let ans2 = vec![(2, 1), (3, 2)];

        assert_eq!(fact1, ans1);
        assert_eq!(fact2, ans2);
    }
}
