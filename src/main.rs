#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn equality_test() {
        assert!(1 == 1)
    }
}
