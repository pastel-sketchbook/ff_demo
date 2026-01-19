#[cfg(feature = "lucky-number")]
fn generate_lucky_number() -> u32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=100)
}

fn main() {
    #[cfg(feature = "print-42")]
    println!("42");

    #[cfg(not(feature = "print-42"))]
    println!("Hello, world!");

    #[cfg(feature = "lucky-number")]
    {
        let lucky = generate_lucky_number();
        println!("Your lucky number: {}", lucky);
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "lucky-number")]
    #[test]
    fn test_lucky_number_in_range() {
        use super::generate_lucky_number;
        for _ in 0..100 {
            let num = generate_lucky_number();
            assert!(num >= 1 && num <= 100);
        }
    }

    #[cfg(not(feature = "lucky-number"))]
    #[test]
    fn test_no_lucky_number_feature() {
        // This test verifies the code compiles without the feature
        assert!(true);
    }
}
