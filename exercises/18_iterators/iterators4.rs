// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.



pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    //take the first number and multiply by the next number down
    //let mut factorial_vec: Vec<u64> = (num..0).collect();
    //factorial_vec = factorial_vec.into_iter().product();
    let mut prod: u64;

    if num >= 1 {
        println!("{:#?}",(num..=1).collect::<Vec<u64>>());
        prod = (1..=num).collect::<Vec<u64>>().into_iter().product();
    } else {
        prod = (num..0).collect::<Vec<u64>>().into_iter().product();
    }
    prod
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
