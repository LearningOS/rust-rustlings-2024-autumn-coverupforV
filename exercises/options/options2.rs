// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    // main function for demonstration purposes
    println!("Run the tests to verify the Option handling.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // Use an if let statement to handle the Option
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        } else {
            panic!("optional_target was None!");
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new(); // Initialize an empty vector

        for i in 0..=range {
            if i == 0 {
                optional_integers.push(None); // Push None for the first element
            } else {
                optional_integers.push(Some(i as i8)); // Push Some(i) for the rest
            }
        }

        let mut cursor = range as i8;

        // while let statement to handle the Option
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
