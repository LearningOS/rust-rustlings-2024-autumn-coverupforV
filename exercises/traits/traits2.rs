// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.



trait AppendBar {
    fn append_bar(self) -> Self;
}

// Implement the trait `AppendBar` for a vector of strings
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut new_vec = self; // Take ownership of the vector
        new_vec.push(String::from("Bar")); // Append "Bar" to the vector
        new_vec // Return the new vector
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar")); // Check if last element is "Bar"
        assert_eq!(foo.pop().unwrap(), String::from("Foo")); // Check if the previous element is "Foo"
    }
}
