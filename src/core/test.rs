#[cfg(test)]
mod test {
    use crate::core::complex::{c32, c64};

    #[test]
    fn test_mul() {
        let a: c32 = c32::new(5, 3);
        let b: c32 = (3, 5).into();

        let c = a * b;
        assert_eq!(c, (15,15).into())
    }

    #[test]
    fn test_ge() {
        let a: c32 = (10,2).into();
        let b: c32 = (3, 5).into();

        assert!(a > b)
    }

    #[test]
    fn test_lt() {
        let a: c32 = (10,2).into();
        let b: c32 = (3, 5).into();

        assert!(b < a)
    }

    #[test]
    fn test_tuple() {
        let a: c32 = (1024,3).into();
        let b: c32 = c32::new(1024, 3);

        assert!(a == b)
    }
    #[test]
    fn test_mul_64() {
        let a: c64 = c64::new(50, 3);
        let b: c64 = (3, 5).into();

        let c = a * b;
        assert_eq!(c, (150,15).into())
    }

    #[test]
    fn test_ge_64() {
        let a: c64 = (10,2).into();
        let b: c64 = (3, 5).into();

        assert!(a > b)
    }

    #[test]
    fn test_lt_64() {
        let a: c64 = (10,2).into();
        let b: c64 = (3, 5).into();

        assert!(b < a)
    }

    #[test]
    fn test_tuple_64() {
        let a: c64 = (1024,3).into();
        let b: c64 = c64::new(1024, 3);

        assert!(a == b)
    }
}
