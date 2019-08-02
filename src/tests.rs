use time::PreciseTime;

#[cfg(test)]
mod tests {
    use prelude::*;
    use time::PreciseTime;

    #[test]
    fn it_works() {
        let x: Symbol = symbol!("x");
        let start = PreciseTime::now();
        let test = function!(x * x + 1.0) * function!(x * x + 1.0);
        let end = PreciseTime::now();
        panic!("{} seconds result is {}", start.to(end), test);
    }
}
