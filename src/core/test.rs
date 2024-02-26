#[cfg(test)]
mod test {
    use crate::core::signed::c32;

    #[test]
    fn test_cplx() {
        let a: c32 = c32::new(5, 3);
        let b: c32 = c32::new(3, 5);

        let c = a * b;
        
    }
}
