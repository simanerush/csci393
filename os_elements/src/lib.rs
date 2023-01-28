pub fn spin_s(num_secs: u64) {
    let now = std::time::Instant::now();
    while now.elapsed().as_secs() < num_secs {
        // spin!
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let now = std::time::Instant::now();
        super::spin_s(3);
        assert!(now.elapsed().as_secs() >= 3);
    }
}
