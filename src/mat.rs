use std::ops::*;
use std::fmt::*;
use std::iter::repeat;

#[derive(Clone)]
pub struct SquareMat {
  a: Box<[f64]>,
  n: usize,
}

impl SquareMat {
  pub fn all(n: usize, x: f64) -> SquareMat {
    SquareMat { a: repeat(x).take(n * n).collect(), n }
  }

  pub fn zeros(n: usize) -> SquareMat {
    assert!(n > 0);
    SquareMat::all(n, 0.0)
  }

  pub fn n(&self) -> usize {
    self.n
  }

  pub fn transpose(&mut self) {
    let n = self.n;
    for i in 0..n {
      for j in i + 1..n {
        let t = self[i][j];
        self[i][j] = self[j][i];
        self[j][i] = t;
      }
    }
  }

  pub fn transpose_copy(&self) -> SquareMat {
    let mut ret = self.clone();
    ret.transpose();
    ret
  }

  pub fn dis_inf(&self, rhs: &SquareMat) -> f64 {
    assert_eq!(self.n, rhs.n);
    let mut ret = 0.0f64;
    let n = self.n;
    // unluckily, f64 doesn't implement Ord, so Iterator::max won't work
    for i in 0..n {
      ret = ret.max((i * n..i * n + n).map(|i| (self.a[i] - rhs.a[i]).abs()).sum());
    }
    ret
  }
}

pub fn vec_dis_inf(a: &[f64], b: &[f64]) -> f64 {
  assert_eq!(a.len(), b.len());
  let mut ret = 0.0f64;
  let n = a.len();
  for i in 0..n {
    ret = ret.max((a[i] - b[i]).abs());
  }
  ret
}

impl Index<usize> for SquareMat {
  type Output = [f64];

  fn index(&self, index: usize) -> &Self::Output {
    let n = self.n;
    &self.a.as_ref()[n * index..n * index + n]
  }
}

impl IndexMut<usize> for SquareMat {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    let n = self.n;
    &mut self.a.as_mut()[n * index..n * index + n]
  }
}

impl<'a, 'b> Mul<&'b SquareMat> for &'a SquareMat {
  type Output = SquareMat;

  fn mul(self, rhs: &'b SquareMat) -> Self::Output {
    assert_eq!(self.n, rhs.n);
    let n = self.n;
    let mut ret = SquareMat::zeros(n);
    for i in 0..n {
      for k in 0..n {
        let t = self[i][k];
        for j in 0..n {
          ret[i][j] += t * rhs[k][j];
        }
      }
    }
    ret
  }
}

impl<'a, 'b> Mul<&'b [f64]> for &'a SquareMat {
  type Output = Box<[f64]>;

  fn mul(self, rhs: &'b [f64]) -> Self::Output {
    use std::iter::repeat;
    assert_eq!(self.n, rhs.len());
    let n = self.n;
    let mut ret = repeat(0.0).take(n).collect::<Box<[_]>>();
    for i in 0..n {
      for j in 0..n {
        ret[i] += self[i][j] * rhs[j];
      }
    }
    ret
  }
}

//impl<'a, 'b> Sub<&'b SquareMat> for &'a SquareMat {
//  type Output = SquareMat;
//
//  fn sub(self, rhs: &'b SquareMat) -> Self::Output {
//    assert_eq!(self.n, rhs.n);
//    let mut ret = self.clone();
//    for i in 0..self.a.len() {
//      ret.a[i] -= rhs.a[i];
//    }
//    ret
//  }
//}

impl Debug for SquareMat {
  fn fmt(&self, f: &mut Formatter) -> Result {
    for i in 0..self.n {
      self[i].fmt(f)?;
      f.write_char('\n')?;
    }
    Ok(())
  }
}