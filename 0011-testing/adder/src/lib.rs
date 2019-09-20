#[derive(Debug)]
pub struct Rect {
    length: u32,
    width: u32,
}

impl Rect {
    pub fn can_hold(&self, other: &Rect) -> bool {
        return self.length > other.length && self.width > other.width;
    }
}

pub fn add_two(a: i32) -> i32 {
    return a + 2;
}

pub fn greeting(name: &str) -> String {
    return format!("Hello {}!", name);
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Out of range {}", value);
        }
        return Guess { value };
    }
}

pub fn add_five(a: i32) -> i32 {
    return internal_adder(a, 5);
}

fn internal_adder(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let l = Rect { length: 8, width: 7 };
        let s = Rect { length: 5, width: 1 };
        assert!(l.can_hold(&s));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let l = Rect { length: 8, width: 7 };
        let s = Rect { length: 5, width: 1 };
        assert!(!s.can_hold(&l));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn explore() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    #[should_panic(expected = "Out of range")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn slow_test() {
        // Not actually slow, yet
    }

    #[test]
    fn test_internal_func() {
        assert_eq!(internal_adder(1, 2), 3);
    }

//    #[test]
//    fn another() {
//        panic!("FAIL! SAD!");
//    }
}
