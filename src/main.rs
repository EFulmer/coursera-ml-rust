use std::env;
use crate::week_one::make_hypothesis;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 4 {
        println!("usage: run theta_0 theta_1 x");
    }

    let theta_0: f64 = args[1].trim().parse().expect(&format!("expected number, got {:?}", args[1]));
    let theta_1: f64 = args[2].trim().parse().expect(&format!("expected number, got {:?}", args[2]));
    let x: f64 = args[3].trim().parse().expect(&format!("expected number, got {:?}", args[3]));
    let hypothesis = make_hypothesis(theta_0, theta_1);
    let x_hat = hypothesis(x);
    println!("{:?} + {:?} * {:?} = {:?}", theta_0, theta_1, x, x_hat);
}

pub mod week_one {
    pub fn make_hypothesis(theta_0: f64, theta_1: f64) -> impl Fn(f64) -> f64 {
        move |x| theta_0 + theta_1 * x
    }
}

#[cfg(test)]
mod test_week_one {

    use crate::week_one::make_hypothesis;

    #[test]
    fn test_make_hypothesis() {
        let hypothesis = make_hypothesis(500.0, 1.5);
        assert_eq!(hypothesis(10.), 515.0);
    }

    #[test]
    fn test_make_zero_hypothesis() {
        let hypothesis = make_hypothesis(0.0, 0.0);
        assert_eq!(hypothesis(1.0), 0.0);
    }

    #[test]
    fn test_make_constant_hypothesis() {
        let hypothesis = make_hypothesis(1.0, 0.0);
        assert_eq!(hypothesis(1.0), 1.0);
        assert_eq!(hypothesis(500_000_000.0), 1.0);
    }

    #[test]
    fn test_hypothesis_returns_theta_1_when_x_0() {
        let theta_0 = 0.14;
        let hypothesis = make_hypothesis(theta_0, 1.0);
        assert_eq!(hypothesis(0.0), theta_0);
    }
}
