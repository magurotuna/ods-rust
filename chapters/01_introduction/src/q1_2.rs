use std::convert::TryFrom;

#[derive(Debug)]
enum One {
    Positive,
    Negative,
}

impl One {
    fn is_positive(&self) -> bool {
        match *self {
            One::Positive => true,
            One::Negative => false,
        }
    }
}

impl TryFrom<i32> for One {
    type Error = &'static str;

    fn try_from(i: i32) -> Result<Self, Self::Error> {
        match i {
            1 => Ok(One::Positive),
            -1 => Ok(One::Negative),
            _ => Err("Cannot convert this integer."),
        }
    }
}

impl From<bool> for One {
    fn from(b: bool) -> Self {
        match b {
            true => One::Positive,
            false => One::Negative,
        }
    }
}

fn is_dyck_word(ones: Vec<One>) -> bool {
    let mut stack = Vec::new();
    for one in ones {
        if stack.is_empty() && !one.is_positive() {
            return false;
        }
        if one.is_positive() {
            stack.push(one);
        } else {
            stack.pop();
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dyck_word() {
        let input = vec![true, false, false, true]
            .into_iter()
            .map(|o| o.into())
            .collect();
        assert!(!is_dyck_word(input));

        let input = vec![true, true, false, false]
            .into_iter()
            .map(|o| o.into())
            .collect();
        assert!(is_dyck_word(input));

        let input = vec![true, false, true, false]
            .into_iter()
            .map(|o| o.into())
            .collect();
        assert!(is_dyck_word(input));
    }
}
