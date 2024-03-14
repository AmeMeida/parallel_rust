
use std::path::Iter;

use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use rayon::prelude::*;

pub trait Round {
    fn ties_to_even(&self) -> Self;

    fn ties_to_odd(&self) -> Self;

    fn ties_to_positive(&self) -> Self;

    fn ties_to_negative(&self) -> Self;
}

impl Round for f64 {
    fn ties_to_even(&self) -> Self {
        let fract = self.fract();

        if fract == 0.5 {
            if self.floor() as i32 & 0b1 == 0 {
                self.floor()
            } else {
                self.ceil()
            }
        } else if fract < 0.5 {
            self.floor()
        } else {
            self.ceil()
        }
    }

    fn ties_to_odd(&self) -> Self {
        let fract = self.fract();

        if fract == 0.5 {
            if self.floor() as i32 & 0b1 == 0 {
                self.ceil()
            } else {
                self.floor()
            }
        } else if fract < 0.5 {
            self.floor()
        } else {
            self.ceil()
        }
    }

    fn ties_to_positive(&self) -> Self {
        let fract = self.fract();

        if fract < 0.5 {
            self.floor()
        } else {
            self.ceil()
        }
    }

    fn ties_to_negative(&self) -> Self {
        let fract = self.fract();

        if fract <= 0.5 {
            self.floor()
        } else {
            self.ceil()
        } 
    }
}

fn main() {
    let mut rng = thread_rng();
    let binding = vec![rng.gen::<f64>(); 10000000000];
    let random_sequences: &[&[f64]] = &vec![binding.as_slice(); 8];

    println!("Generated random numbers.");

    let cum = random_sequences.par_iter().map(|sequence| {
        sequence.iter().map(|float| {
            let rounded = float.floor();

            let diff = rounded - float;

            diff
        }).sum::<f64>()
    }).sum::<f64>();

    println!("Average difference: {}", cum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ties_to_even() {
        assert_eq!(3.5f64.ties_to_even(), 4.0);
        assert_eq!(4.5f64.ties_to_even(), 4.0);
        assert_eq!(5.5f64.ties_to_even(), 6.0);
        assert_eq!(6.5f64.ties_to_even(), 6.0);

        assert_eq!(3.4f64.ties_to_even(), 3.0);
        assert_eq!(3.6f64.ties_to_even(), 4.0);
    }

    #[test]
    fn test_ties_to_odd() {
        assert_eq!(3.5f64.ties_to_odd(), 3.0);
        assert_eq!(4.5f64.ties_to_odd(), 5.0);
        assert_eq!(5.5f64.ties_to_odd(), 5.0);
        assert_eq!(6.5f64.ties_to_odd(), 7.0);

        assert_eq!(3.4f64.ties_to_odd(), 3.0);
        assert_eq!(3.6f64.ties_to_odd(), 4.0);
    }

    #[test]
    fn test_ties_to_positive() {
        assert_eq!(3.5f64.ties_to_positive(), 4.0);
        assert_eq!(4.5f64.ties_to_positive(), 5.0);
        assert_eq!(5.5f64.ties_to_positive(), 6.0);
        assert_eq!(6.5f64.ties_to_positive(), 7.0);
        assert_eq!(3.4f64.ties_to_positive(), 3.0);
        assert_eq!(3.6f64.ties_to_positive(), 4.0);
    }

    #[test]
    fn test_ties_to_negative() {
        assert_eq!(3.5f64.ties_to_negative(), 3.0);
        assert_eq!(4.5f64.ties_to_negative(), 4.0);
        assert_eq!(5.5f64.ties_to_negative(), 5.0);
        assert_eq!(6.5f64.ties_to_negative(), 6.0);
        assert_eq!(3.4f64.ties_to_negative(), 3.0);
        assert_eq!(3.6f64.ties_to_negative(), 4.0);
    }

    #[test]
    fn test_floor() {
        assert_eq!(3.5f64.floor(), 3.0);
        assert_eq!(4.5f64.floor(), 4.0);
        assert_eq!(5.5f64.floor(), 5.0);
        assert_eq!(6.5f64.floor(), 6.0);

        assert_eq!(3.4f64.floor(), 3.0);
        assert_eq!(3.6f64.floor(), 3.0);
    }

    #[test]
    fn test_ceil() {
        assert_eq!(3.5f64.ceil(), 4.0);
        assert_eq!(4.5f64.ceil(), 5.0);
        assert_eq!(5.5f64.ceil(), 6.0);
        assert_eq!(6.5f64.ceil(), 7.0);

        assert_eq!(3.4f64.ceil(), 4.0);
        assert_eq!(3.6f64.ceil(), 4.0);
    }
}
