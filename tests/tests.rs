#[cfg(test)]
mod tests {
    use rust_fun::*;
    #[test]
    fn test_c_to_f() {
        assert_eq!(c_to_f(0.0), 32.0);
        assert_eq!(c_to_f(100.0), 212.0);
    }

    #[test]
    fn test_f_to_c() {
        assert_eq!(f_to_c(32.0), 0.0);
        assert_eq!(f_to_c(55.5), 13.055556);
    }

    #[test]
    fn test_add() {
        assert_eq!(add(10, 10), 20);
        assert_eq!(add(-5, 10), 5);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(5, 10), -5);
        assert_eq!(sub(100, 100), 0);
    }

    #[test]
    fn test_mult() {
        assert_eq!(mult(7, 8), 56);
        assert_eq!(mult(5, -10), -50);
    }
}