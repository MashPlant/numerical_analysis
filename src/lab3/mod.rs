pub mod q6 {
  use crate::mat::SquareMat;
  use crate::mat::vec_dis_inf;

  pub fn make_hilbert(n: usize) -> SquareMat {
    let mut ret = SquareMat::zeros(n);
    for i in 0..n {
      for j in 0..n {
        ret[i][j] = 1.0 / (i + j + 1) as f64;
      }
    }
    ret
  }

  pub fn cholesky(a: &mut SquareMat) {
    let n = a.n();
    for j in 0..n {
      for k in 0..j {
        a[j][j] -= a[j][k] * a[j][k];
      }
      a[j][j] = a[j][j].sqrt();
      for i in j + 1..n {
        for k in 0..j {
          a[i][j] -= a[i][k] * a[j][k];
        }

        a[i][j] /= a[j][j];
      }
      for i in j + 1..n {
        a[j][i] = 0.0;
      }
    }
  }

  pub fn cholesky_copy(a: &SquareMat) -> SquareMat {
    let mut ret = a.clone();
    cholesky(&mut ret);
    ret
  }

  pub fn solve_positive_definite_inplace(a: &mut SquareMat, b: &mut [f64]) {
    assert_eq!(a.n(), b.len());
    cholesky(a);
    let n = a.n();
    for i in 0..n {
      for j in 0..i {
        b[i] -= b[j] * a[i][j];
      }
      b[i] /= a[i][i];
    }
    a.transpose();
    for i in (0..n).rev() {
      for j in i + 1..n {
        b[i] -= b[j] * a[i][j];
      }
      b[i] /= a[i][i];
    }
  }

  pub fn solve_positive_definite(a: &SquareMat, b: &[f64]) -> Box<[f64]> {
    let mut b = Box::from(b);
    solve_positive_definite_inplace(&mut a.clone(), &mut b);
    b
  }

  pub fn solve() {
    use std::iter::repeat;
    let h = make_hilbert(10);
    let x = repeat(1.0).take(10).collect::<Box<[_]>>();
    let mut b = &h * x.as_ref();
    let xs = solve_positive_definite(&h, &b);
    let bs = &h * xs.as_ref();
    println!("{:?}", vec_dis_inf(&xs, &x));
    println!("{:?}", vec_dis_inf(&bs, &b));
    for b in b.iter_mut() {
      *b += 1e-7;
    }
    let xs = solve_positive_definite(&h, &b);
    let bs = &h * xs.as_ref();
    println!("{:?}", vec_dis_inf(&xs, &x));
    println!("{:?}", vec_dis_inf(&bs, &b));
  }
}