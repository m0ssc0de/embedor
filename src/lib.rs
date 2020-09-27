pub fn haha() {
    println!("haha");
}

#[cfg(test)]
mod tests {
    use super::haha;
    #[test]
    fn it_works() {
        haha();
        assert_eq!(2 + 2, 4);
    }
}
