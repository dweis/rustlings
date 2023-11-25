// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// COME BACK TO THIS IN THE FUTURE


struct Factorial {
    last: u64,
    n: u64,
}

impl Factorial {
    fn new() -> Self {
        Self {
            last: 1,
            n: 0
        }
    }
}

impl Iterator for Factorial {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 || self.n == 1 {
            self.last = 1;
        } else {
            self.last = self.n * self.last;
        }

        self.n += 1;

        Some(self.last)
    }
}

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Concise iterator style
    (1..=num).product()
    // Iterator style:
    // *Factorial::new()
    //     .skip(num as usize)
    //     .take(1)
    //     .collect::<Vec<u64>>()
    //     .first()
    //     .unwrap()
    // Recursive style:
    // if num == 0 {
    //     1
    // } else if num > 0 {
    //     factorial(num-1) * num
    // } 
    // Execute `rustlings hint iterators4` for hints.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
