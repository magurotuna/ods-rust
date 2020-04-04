fn reverse<T>(mut stack: Vec<T>) -> Vec<T> {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    while let Some(v) = stack.pop() {
        queue.push_back(v);
    }
    queue.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(vec![1, 2, 3]), vec![3, 2, 1]);
        assert_eq!(reverse(vec![1]), vec![1]);
        assert_eq!(reverse(vec![3, 2, 1]), vec![1, 2, 3]);
    }
}
