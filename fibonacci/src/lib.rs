/// fibonacci provides nth fibonacci number
pub fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n <= 2 {
        return 1;
    }
    let mut next: u64 = 1;
    let mut before = 1;
    for _ in 2..n {
        next = before + next;
        before = next-before;
    }
    next
}


#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn test_fibonacci() {
        struct TestCase {
            given: u64,
            expected: u64,
        }
        let test_cases = vec![
            TestCase{given: 1, expected: 1},
            TestCase{given: 2, expected: 1},
            TestCase{given: 3, expected: 2},
            TestCase{given: 4, expected: 3},
            TestCase{given: 5, expected: 5},
            TestCase{given: 6, expected: 8},
            TestCase{given: 7, expected: 13},
            TestCase{given: 8, expected: 21},
            TestCase{given: 9, expected: 34},
            TestCase{given: 30, expected: 832040},
            TestCase{given: 40, expected: 102334155},
            TestCase{given: 50, expected: 12586269025},
            TestCase{given: 55, expected: 139583862445},
            TestCase{given: 60, expected: 1548008755920},
            TestCase{given: 70, expected: 190392490709135},
            TestCase{given: 77, expected: 5527939700884757},
            TestCase{given: 80, expected: 23416728348467685},
            TestCase{given: 85, expected: 259695496911122585},
        ];
        for i in 0..test_cases.len() {
            let res = fibonacci(test_cases[i].given);
            assert!(res == test_cases[i].expected);
        }
    }

}
