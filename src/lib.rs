use num_traits::*;

pub fn lerp<T: Float> (a: T, b: T, f: T) -> T {
    a + (b - a) * f
}

pub fn inverse_lerp<T: Float> (a: T, b: T, v: T) -> T {
    (v - a) / (b - a)
}

#[cfg(test)]
mod tests {
    #[test]
    fn positive_lerp() {
        assert_eq!(crate::lerp(1.0, 2.0, 0.5), 1.5)
    }
    #[test]
    fn positive_reverse_lerp() {
        assert_eq!(crate::lerp(10.0, 0.0, 0.5), 5.0)
    }
    #[test]
    fn negative_lerp() {
        assert_eq!(crate::lerp(-10., 0., 0.5), -5.)
    }
    #[test]
    fn reverse_negative_lerp() {
        assert_eq!(crate::lerp(0., -10., 0.5), -5.)
    }
    #[test]
    fn basic_inverse_lerp() {
        assert_eq!(crate::inverse_lerp(10.0, 5.0, 7.5), 0.5)
    }
}