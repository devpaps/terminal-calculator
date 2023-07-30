#[cfg(test)]
mod tests {
    use crate::{Arithmetic, History, Numbers};

    #[test]
    fn test_add() {
        let mut history = History {
            operations: Vec::new(),
        };
        let mut numbers = Numbers {
            num_one: 1.0,
            num_two: 2.0,
        };
        let result = numbers.sum(&mut history);
        assert_eq!(result, 3.0);
    }
}
