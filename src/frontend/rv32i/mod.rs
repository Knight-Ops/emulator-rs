pub mod cpu;
mod instructions;
mod mem;
mod registers;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
