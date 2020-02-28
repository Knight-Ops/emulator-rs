mod decoder;
mod registers;
mod mem;
pub mod CPU;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
