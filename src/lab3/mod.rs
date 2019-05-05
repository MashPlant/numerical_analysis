pub mod q6 {
  use crate::square_mat::*;

  pub fn gauss(a: &mut SquareMat, b: &mut [f64]) {
    assert_eq!(a.n(), b.len());
    let n = a.n();
    for k in 0..n {
      let mut max = a[k][k].abs();
      let mut which = k;
      for i in k + 1..n {
        if a[i][k].abs() > max {
          max = a[i][k].abs();
          which = i;
        }
      }
      macro_rules! swap {
        ($a:expr, $b: expr) => {
           let tmp = $a;
           $a = $b;
           $b = tmp;
        };
      }
      for i in k..n {
        swap!(a[which][i], a[k][i]);
      }
      swap!(b[which], b[k]);
      for i in k + 1..n {
        let fac = -a[i][k] / a[k][k];
        for j in k..n {
          a[i][j] += fac * a[k][j];
        }
        b[i] += fac * b[k];
      }
    }
    for i in (0..n).rev() {
      b[i] = (b[i] - b[i + 1..n].iter().zip(a[i][i + 1..n].iter()).map(|(&a, &b)| a * b).sum::<f64>()) / a[i][i];
    }
  }

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
    for &n in &[10, 8, 12] {
      let h = make_hilbert(n);
      let x = repeat(1.0).take(n).collect::<Box<[_]>>();
      let mut b = &h * x.as_ref();

      println!("n = {}", n);
      println!("before disturbance");
      let xs = solve_positive_definite(&h, &b);
      let bs = &h * xs.as_ref();
      println!("solve = {:?}", xs);
      println!("inf norm of delta b = {:?}", vec_dis_inf(&bs, &b));
      println!("inf norm of delta x = {:?}", vec_dis_inf(&xs, &x));

      for b in b.iter_mut() { *b += 1e-7; }

      println!("after disturbance");
      let xs = solve_positive_definite(&h, &b);
      let bs = &h * xs.as_ref();
      println!("inf norm of delta b = {:?}", vec_dis_inf(&bs, &b));
      println!("inf norm of delta x = {:?}", vec_dis_inf(&xs, &x));

      println!("problem cond = {:?}", (vec_dis_inf(&xs, &x) / vec_norm_inf(&x)) / (1e-7 / vec_norm_inf(&b)));

      println!();
    }
  }
}