use std::io::{BufRead, Write};

trait Solver {
    fn solve<R: BufRead, W: Write>(reader: R, writer: &mut W);
}

struct Q1Solver;
impl Solver for Q1Solver {
    fn solve<R: BufRead, W: Write>(reader: R, writer: &mut W) {
        let mut stack = Vec::new();
        for line in reader.lines() {
            stack.push(line.unwrap());
        }

        while let Some(rev_line) = stack.pop() {
            writeln!(writer, "{}", rev_line).unwrap();
        }
    }
}

struct Q2Solver;
impl Solver for Q2Solver {
    fn solve<R: BufRead, W: Write>(reader: R, writer: &mut W) {
        let mut stack = Vec::with_capacity(50);
        for line in reader.lines() {
            if stack.len() == 50 {
                while let Some(rev_line) = stack.pop() {
                    writeln!(writer, "{}", rev_line).unwrap();
                }
            }
            stack.push(line.unwrap());
        }
        while let Some(rev_line) = stack.pop() {
            writeln!(writer, "{}", rev_line).unwrap();
        }
    }
}

struct Q3Solver;
impl Solver for Q3Solver {
    fn solve<R: BufRead, W: Write>(reader: R, writer: &mut W) {
        use std::collections::VecDeque;
        let mut queue = VecDeque::with_capacity(42);
        for line in reader.lines() {
            let line = line.unwrap();
            if queue.len() == 42 {
                let front = queue.pop_front().unwrap();
                if line.is_empty() {
                    writeln!(writer, "{}", front).unwrap();
                }
            }
            queue.push_back(line);
        }
    }
}

struct Q4Solver;
impl Solver for Q4Solver {
    fn solve<R: BufRead, W: Write>(reader: R, writer: &mut W) {
        use std::collections::HashSet;
        let mut appeared = HashSet::new();
        for line in reader.lines() {
            let line = line.unwrap();
            if !appeared.contains(&line) {
                writeln!(writer, "{}", &line).unwrap();
                appeared.insert(line);
            }
        }
    }
}

struct Q5Solver;
impl Solver for Q5Solver {
    fn solve<R: BufRead, W: Write>(reader: R, writer: &mut W) {
        use std::collections::HashSet;
        let mut appeared = HashSet::new();
        for line in reader.lines() {
            let line = line.unwrap();
            if appeared.contains(&line) {
                writeln!(writer, "{}", &line).unwrap();
            } else {
                appeared.insert(line);
            }
        }
    }
}

struct Q6Solver;
impl Solver for Q6Solver {
    fn solve<R: BufRead, W: Write>(reader: R, writer: &mut W) {
        use std::collections::BTreeSet;
        let mut sorted_set = BTreeSet::new();
        for line in reader.lines() {
            let line = line.unwrap();
            sorted_set.insert((line.len(), line));
        }

        for res in sorted_set {
            writeln!(writer, "{}", res.1).unwrap();
        }
    }
}

struct Q7Solver;
impl Solver for Q7Solver {
    fn solve<R: BufRead, W: Write>(reader: R, writer: &mut W) {
        use std::collections::BTreeMap;
        let mut sorted_map = BTreeMap::new();
        for line in reader.lines() {
            let line = line.unwrap();
            *sorted_map.entry((line.len(), line)).or_insert(0) += 1;
        }

        for (res, count) in sorted_map {
            for _ in 0..count {
                writeln!(writer, "{}", res.1).unwrap();
            }
        }
    }
}

struct Q8Solver;
impl Solver for Q8Solver {
    fn solve<R: BufRead, W: Write>(reader: R, writer: &mut W) {
        let mut odds = Vec::new();
        for (i, line) in reader.lines().enumerate() {
            if i % 2 == 0 {
                writeln!(writer, "{}", line.unwrap());
            } else {
                odds.push(line.unwrap());
            }
        }
        odds.into_iter()
            .for_each(|line| writeln!(writer, "{}", line).unwrap());
    }
}

struct Q9Solver;
impl Solver for Q9Solver {
    fn solve<R: BufRead, W: Write>(reader: R, writer: &mut W) {
        use rand::prelude::*;
        let mut rng = thread_rng();
        let mut input: Vec<_> = reader.lines().map(|l| l.unwrap()).collect();
        input.shuffle(&mut rng);

        input
            .into_iter()
            .for_each(|line| writeln!(writer, "{}", line).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut input = String::new();
        for i in 0..1000 {
            input.push_str(format!("line {}\n", i).as_str());
        }
        let mut expected = String::new();
        for i in (0..1000).rev() {
            expected.push_str(format!("line {}\n", i).as_str());
        }

        let mut writer = Vec::new();
        Q1Solver::solve(input.as_bytes(), &mut writer);
        assert_eq!(writer, expected.as_bytes());
    }

    #[test]
    fn test_2_100lines() {
        let mut input = String::new();
        for i in 0..2 {
            for j in 0..50 {
                input.push_str(format!("line {}\n", i * 50 + j).as_str());
            }
        }

        let mut expected = String::new();
        for i in 0..2 {
            for j in 0..50 {
                expected.push_str(format!("line {}\n", (i + 1) * 50 - (j + 1)).as_str());
            }
        }

        let mut writer = Vec::new();
        Q2Solver::solve(input.as_bytes(), &mut writer);
        assert_eq!(writer, expected.as_bytes());
    }

    #[test]
    fn test_2_51lines() {
        let mut input = String::new();
        for i in 0..51 {
            input.push_str(format!("line {}\n", i).as_str());
        }

        let mut expected = String::new();
        for i in (0..50).rev() {
            expected.push_str(format!("line {}\n", i).as_str());
        }
        expected.push_str(format!("line {}\n", 50).as_str());

        let mut writer = Vec::new();
        Q2Solver::solve(input.as_bytes(), &mut writer);
        assert_eq!(writer, expected.as_bytes());
    }

    #[test]
    fn test_2_99lines() {
        let mut input = String::new();
        for i in 0..99 {
            input.push_str(format!("line {}\n", i).as_str());
        }

        let mut expected = String::new();
        for i in (0..50).rev() {
            expected.push_str(format!("line {}\n", i).as_str());
        }
        for i in (50..99).rev() {
            expected.push_str(format!("line {}\n", i).as_str());
        }

        let mut writer = Vec::new();
        Q2Solver::solve(input.as_bytes(), &mut writer);
        assert_eq!(writer, expected.as_bytes());
    }

    #[test]
    fn test_3() {
        let mut input = String::new();
        for i in 0..84 {
            if i < 42 || i % 2 == 0 {
                input.push_str(format!("line {}\n", i).as_str());
            } else {
                input.push_str("\n");
            }
        }
        let mut expected = String::new();
        for i in (0..42).filter(|x| x % 2 != 0) {
            expected.push_str(format!("line {}\n", i).as_str());
        }

        let mut writer = Vec::new();
        Q3Solver::solve(input.as_bytes(), &mut writer);
        assert_eq!(writer, expected.as_bytes());
    }

    #[test]
    fn test_4() {
        let input = r"
foo
bar
baz
hoge
piyo
foo
hoge
bar
";

        let expected = r"
foo
bar
baz
hoge
piyo
";

        let mut writer = Vec::new();
        Q4Solver::solve(input.as_bytes(), &mut writer);
        assert_eq!(writer, expected.as_bytes());
    }

    #[test]
    fn test_5() {
        let input = r"foo
bar
baz
hoge
piyo
foo
hoge
bar
foo
";

        let expected = r"foo
hoge
bar
foo
";

        let mut writer = Vec::new();
        Q5Solver::solve(input.as_bytes(), &mut writer);
        assert_eq!(writer, expected.as_bytes());
    }

    #[test]
    fn test_6() {
        let input = r"a
bc
aa
ab
aaa
ba
b
c
aa
";

        let expected = r"a
b
c
aa
ab
ba
bc
aaa
";

        let mut writer = Vec::new();
        Q6Solver::solve(input.as_bytes(), &mut writer);
        assert_eq!(writer, expected.as_bytes());
    }

    #[test]
    fn test_7() {
        let input = r"a
bc
aa
ab
aaa
ba
b
c
aa
";

        let expected = r"a
b
c
aa
aa
ab
ba
bc
aaa
";

        let mut writer = Vec::new();
        Q7Solver::solve(input.as_bytes(), &mut writer);
        assert_eq!(writer, expected.as_bytes());
    }

    #[test]
    fn test_8() {
        let mut input = String::new();
        for i in 0..100 {
            input.push_str(format!("line {}\n", i).as_str());
        }

        let mut expected = String::new();
        for i in (0..100).filter(|x| x % 2 == 0) {
            expected.push_str(format!("line {}\n", i).as_str());
        }
        for i in (0..100).filter(|x| x % 2 != 0) {
            expected.push_str(format!("line {}\n", i).as_str());
        }

        let mut writer = Vec::new();
        Q8Solver::solve(input.as_bytes(), &mut writer);
        assert_eq!(writer, expected.as_bytes());
    }

    #[test]
    fn test_9() {
        use std::collections::HashSet;
        let mut input = String::new();
        let mut expected_set = HashSet::new();
        for i in 0..100 {
            input.push_str(format!("line {}\n", i).as_str());
            expected_set.insert(format!("line {}", i));
        }

        let mut writer = Vec::new();
        Q9Solver::solve(input.as_bytes(), &mut writer);

        for line in std::str::from_utf8(&writer).unwrap().to_string().lines() {
            assert!(expected_set.contains(line));
        }
    }
}
