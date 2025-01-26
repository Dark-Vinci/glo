mod mocks;
// mod third;x

pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDivide {
        a: i32,
        b: i32,
        result: Option<i32>
    }

    impl TestDivide {
        fn new(a: i32, b: i32, c: Option<i32>) -> Self {
            TestDivide { a, b, result: c }
        }
    }

    #[test]
    fn test_divide_with_loop() {
        let cases = vec![
            TestDivide::new(6, 2, Some(3)),
            TestDivide::new(10, 5, Some(2)),
            TestDivide::new(1, 0, None),
        ];

        for TestDivide { a, b, result } in cases {
            assert_eq!(divide(a, b), result, "Failed on inputs: ({}, {})", a, b);
        }
    }
}

fn main() {}