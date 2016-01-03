use num;
use num::traits::Float;

use traits::Filter;

/// A single channel, two pole digital filter.
///
/// A `TwoPole` filter uses the following equation:
///
/// `y[n] = b0*x[n] - a1*y[n-1] - a2*x[n-2]`
///
/// It has two feedback coefficients, `a1` and `a2`. 
pub struct TwoPole<T> {
  y_z1: T,
  y_z2: T,
  pub b0: T,
  pub a1: T,
  pub a2: T
}

impl<T> TwoPole<T> where T: Float {
  /// Creates a new `TwoPole` filter.
  ///
  /// The filter will be initalized in a state that does not alter the input
  /// signal.
  ///
  /// # Examples
  ///
  /// ```
  /// # #![allow(unused_mut)]
  /// use rasp::filter::TwoPole;
  ///
  /// let mut filter1: TwoPole<f32> = TwoPole::new();
  /// let mut filter2: TwoPole<f64> = TwoPole::new();
  /// let mut filter3 = TwoPole::<f32>::new();
  /// let mut filter4 = TwoPole::<f64>::new();
  /// ```
  pub fn new() -> Self {
    TwoPole {
      y_z1: num::zero(),
      y_z2: num::zero(),
      b0: num::one(),
      a1: num::zero(),
      a2: num::zero()
    }
  }

  /// Sets all filter coefficients at once.
  ///
  /// `a1` and `a2` are feedbacks, or poles.
  pub fn set_coefficients(&mut self, b0: T, a1: T, a2: T) {
    self.b0 = b0;
    self.a1 = a1;
    self.a2 = a2;
  }
}

impl<T> Filter<T> for TwoPole<T> where T: Float {
  fn tick(&mut self, sample: T) -> T {
    let output = self.b0 * sample
      - self.a1 * self.y_z1 - self.a2 * self.y_z2;
    self.y_z2 = self.y_z1;
    self.y_z1 = output;
    output
  }

  fn clear(&mut self) {
    self.y_z1 = num::zero();
    self.y_z2 = num::zero();
  }

  fn last_out(&self) -> T {
    self.y_z1
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::EPSILON;
  use ::traits::Filter;

  #[test]
  fn tick() {
    let input = vec![0.55f32, -0.55f32, 0.55f32, -0.55f32 /*, 0.25f32*/];
    let expected =
      vec![
         0.495_000_000_000f32,
        -0.594_000_000_000f32,
         1.257_300_000_000f32,
        -1.518_660_000_000f32,
         2.163_222_000_000f32
      ];
    let mut two_pole = TwoPole::new();
    for sample in input.iter() {
      assert!((two_pole.tick(*sample) - sample).abs() <= EPSILON);
    }
    two_pole.clear();
    two_pole.set_coefficients(0.9f32, 0.2, -1.3f32);
    for i in 0..input.len() {
      let output = two_pole.tick(input[i]);
      println!("{:.12} - {:.12} = {:.12}", expected[i], output, expected[i] - output);
      assert!((expected[i] - output).abs() <= EPSILON);
    }
  }
}
