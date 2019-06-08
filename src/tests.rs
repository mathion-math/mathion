use time::PreciseTime;

#[cfg(test)]
mod tests {
    use prelude::*;
    use time::PreciseTime;

    #[test]
    fn it_works() {
        let x: Symbol = symbol!("x");
        let start = PreciseTime::now();
        let test = function!(5.0 * x.powf(2.0) + 2.0 * x + 4.0) * function!(5.0 * x.powf(2.0) + 2.0 * x + 4.0);
        let end = PreciseTime::now();
        panic!("{} seconds result is {}", start.to(end), test);
    }
}
