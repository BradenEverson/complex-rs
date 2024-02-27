#[cfg(test)]
mod test {
    use crate::core::complex::c32;

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
}
