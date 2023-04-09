pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// you can test private functions since child modules can access parent modules' functions
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// functions under cfg(test) will not be included in the binary
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
