pub mod bsearch;
pub mod btree;
pub mod buffer;
pub mod config;
pub mod disk;
pub mod slotted;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4usize as i64);
    }
}
