use cargo_snippet::snippet;

#[snippet(name = "vector_op")]
pub mod vector {

    use std::ops::{Add, Mul, Sub};

    #[derive(Debug)]
    pub struct Vector2d<T> {
        x: T,
        y: T,
    }

    impl<T> Vector2d<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        pub fn new(x: T, y: T) -> Vector2d<T> {
            Vector2d { x, y }
        }

        pub fn add(self, v: Vector2d<T>) -> Vector2d<T> {
            Vector2d {
                x: self.x + v.x,
                y: self.y + v.y,
            }
        }

        pub fn sub(self, v: Vector2d<T>) -> Vector2d<T> {
            Vector2d {
                x: self.x - v.x,
                y: self.y - v.y,
            }
        }

        pub fn inner_product(self, v: Vector2d<T>) -> T {
            self.x * v.x + self.y * v.y
        }

        pub fn cross(self, v: Vector2d<T>) -> T {
            self.x * v.y - self.y * v.x
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Vector2d;

        // 全型共通の基本テストを生成する
        macro_rules! vector2d_common_tests {
            ($mod_name:ident, $t:ty) => {
                mod $mod_name {
                    use super::Vector2d;

                    #[test]
                    fn test_new() {
                        let vec = Vector2d::new(3 as $t, 5 as $t);
                        assert_eq!(vec.x, 3 as $t);
                        assert_eq!(vec.y, 5 as $t);
                    }

                    #[test]
                    fn test_add() {
                        let left = Vector2d::new(3 as $t, 5 as $t);
                        let right = Vector2d::new(1 as $t, 2 as $t);
                        let result = left.add(right);
                        assert_eq!(result.x, 4 as $t);
                        assert_eq!(result.y, 7 as $t);
                    }

                    #[test]
                    fn test_sub() {
                        let left = Vector2d::new(5 as $t, 7 as $t);
                        let right = Vector2d::new(1 as $t, 2 as $t);
                        let result = left.sub(right);
                        assert_eq!(result.x, 4 as $t);
                        assert_eq!(result.y, 5 as $t);
                    }

                    #[test]
                    fn test_inner_product() {
                        let left = Vector2d::new(3 as $t, 4 as $t);
                        let right = Vector2d::new(2 as $t, 5 as $t);
                        assert_eq!(left.inner_product(right), 26 as $t);
                    }

                    #[test]
                    fn test_cross() {
                        let left = Vector2d::new(3 as $t, 1 as $t);
                        let right = Vector2d::new(2 as $t, 4 as $t);
                        assert_eq!(left.cross(right), 10 as $t);
                    }

                    #[test]
                    fn test_add_zero() {
                        let left = Vector2d::new(3 as $t, 5 as $t);
                        let zero = Vector2d::new(0 as $t, 0 as $t);
                        let result = left.add(zero);
                        assert_eq!(result.x, 3 as $t);
                        assert_eq!(result.y, 5 as $t);
                    }

                    #[test]
                    fn test_orthogonal_inner_product() {
                        let left = Vector2d::new(1 as $t, 0 as $t);
                        let right = Vector2d::new(0 as $t, 1 as $t);
                        assert_eq!(left.inner_product(right), 0 as $t);
                    }

                    #[test]
                    fn test_parallel_cross() {
                        let left = Vector2d::new(2 as $t, 4 as $t);
                        let right = Vector2d::new(1 as $t, 2 as $t);
                        assert_eq!(left.cross(right), 0 as $t);
                    }
                }
            };
        }

        // 符号あり整数型の負値テストを生成する
        macro_rules! vector2d_signed_tests {
            ($mod_name:ident, $t:ty) => {
                mod $mod_name {
                    use super::Vector2d;

                    #[test]
                    fn test_add_negative() {
                        let left = Vector2d::new(-3 as $t, 5 as $t);
                        let right = Vector2d::new(1 as $t, -2 as $t);
                        let result = left.add(right);
                        assert_eq!(result.x, -2 as $t);
                        assert_eq!(result.y, 3 as $t);
                    }

                    #[test]
                    fn test_sub_negative_result() {
                        let left = Vector2d::new(1 as $t, 2 as $t);
                        let right = Vector2d::new(3 as $t, 5 as $t);
                        let result = left.sub(right);
                        assert_eq!(result.x, -2 as $t);
                        assert_eq!(result.y, -3 as $t);
                    }

                    #[test]
                    fn test_inner_product_negative() {
                        let left = Vector2d::new(-1 as $t, 2 as $t);
                        let right = Vector2d::new(3 as $t, -4 as $t);
                        assert_eq!(left.inner_product(right), -11 as $t);
                    }

                    #[test]
                    fn test_cross_negative() {
                        let left = Vector2d::new(1 as $t, 4 as $t);
                        let right = Vector2d::new(3 as $t, 2 as $t);
                        assert_eq!(left.cross(right), -10 as $t);
                    }
                }
            };
        }

        vector2d_common_tests!(test_i64, i64);
        vector2d_common_tests!(test_isize, isize);
        vector2d_common_tests!(test_i128, i128);
        vector2d_common_tests!(test_f64, f64);

        vector2d_signed_tests!(test_signed_i64, i64);
        vector2d_signed_tests!(test_signed_isize, isize);
        vector2d_signed_tests!(test_signed_i128, i128);

        // f64の浮動小数点精度に関するテスト
        mod test_f64_precision {
            use super::Vector2d;

            const EPSILON: f64 = 1e-10;

            #[test]
            fn test_inner_product_fractional() {
                let left = Vector2d::new(0.1_f64, 0.2_f64);
                let right = Vector2d::new(0.3_f64, 0.4_f64);
                let result = left.inner_product(right);
                assert!((result - 0.11_f64).abs() < EPSILON);
            }

            #[test]
            fn test_cross_negative() {
                let left = Vector2d::new(1.5_f64, 3.5_f64);
                let right = Vector2d::new(4.5_f64, 2.5_f64);
                assert_eq!(left.cross(right), -12.0_f64);
            }
        }
    }
}
