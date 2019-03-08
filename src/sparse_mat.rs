use std::fmt::*;
use crate::square_mat::SquareMat;

// Box<[(a[i][i], Vec<(other nonzero elem, its index)>)]>
pub struct SparseMat(Box<[(f64, Vec<(f64, usize)>)]>);

impl SparseMat {
  pub fn from_diagonal(diagonal_gen: impl Iterator<Item=f64>) -> SparseMat {
    SparseMat(diagonal_gen.map(|x| (x, Vec::new())).collect())
  }

  pub fn add(&mut self, row: usize, col: usize, x: f64) {
    self.0[row].1.push((x, col));
  }

  pub fn n(&self) -> usize {
    self.0.len()
  }

  pub fn diagonal_at(&self, row: usize) -> f64 {
    self.0[row].0
  }

  // diagonal is not included
  pub fn row_at(&self, row: usize) -> impl Iterator<Item=&(f64, usize)> {
    self.0[row].1.iter()
  }

  pub fn to_dense(&self) -> SquareMat {
    let mut ret = SquareMat::zeros(self.n());
    for (i, (x, v)) in self.0.as_ref().iter().enumerate() {
      ret[i][i] = *x;
      for (x, col) in v {
        ret[i][*col] = *x;
      }
    }
    ret
  }
}

impl Debug for SparseMat {
  fn fmt(&self, f: &mut Formatter) -> Result {
    for (i, (x, v)) in self.0.as_ref().iter().enumerate() {
      write!(f, "row {:?}: diagonal = {:?}, other = {:?}\n", i, x, v)?;
    }
    Ok(())
  }
}