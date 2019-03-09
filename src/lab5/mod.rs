pub mod q1 {
  use crate::square_mat::SquareMat;
  use crate::rand::{self, Rng};

  pub fn power_method(a: &SquareMat, eps: f64) -> (f64, Box<[f64]>) {
    let n = a.n();
    let mut rng = rand::thread_rng();
    let mut v = (0..n).map(|_| rng.gen()).collect::<Box<[f64]>>();
    let mut u = v.clone();
    let mut old_max = 0.0;
    loop {
      v = a * u.as_ref();
      let mut max = 0.0f64;
      for v in v.iter() {
        max = max.max(v.abs());
      }
      for (v, u) in v.iter().zip(u.iter_mut()) {
        *u = *v / max;
      }
      if (max - old_max).abs() < eps {
        break (max, u);
      }
      old_max = max;
    }
  }

  pub fn solve() {
    let a = SquareMat::from_slice(&[5.0, -4.0, 1.0, -4.0, 6.0, -4.0, 1.0, -4.0, 7.0]);
    println!("{:?}", power_method(&a, 1e-5));
    let a = SquareMat::from_slice(&[25.0, -41.0, 10.0, -6.0, -41.0, 68.0, -17.0, 10.0, 10.0, -17.0, 5.0, -3.0, -6.0, 10.0, -3.0, 2.0]);
    println!("{:?}", power_method(&a, 1e-5));
  }
}

pub mod q3 {
  use crate::square_mat::{SquareMat, vec_dot};

  pub fn make_household(a: &mut SquareMat, v: &[f64]) {
    assert_eq!(a.n(), v.len());
    let n = a.n();
    for i in 0..n {
      for j in 0..n {
        a[i][j] = -2.0 * v[i] * v[j];
      }
    }
    for i in 0..n { a[i][i] += 1.0; }
  }

  // a will be R, ret will be Q
  pub fn qr(a: &mut SquareMat) -> SquareMat {
    let n = a.n();
    a.transpose(); // convenient to calculate with col major
    let mut v = SquareMat::zeros(n);
    for k in 0..n {
      // 2-norm of lower triangle
      let sigma = a[k][k].signum() * a[k].iter().skip(k).map(|x| *x * *x).sum::<f64>().sqrt();
      if sigma == a[k][k] { continue; } // col k is all 0 before diagonal
      for j in k..n { v[k][j] = a[k][j]; }
      v[k][k] += sigma;
      let beta = vec_dot(&v[k], &v[k]);
      for j in k..n {
        let gamma = 2.0 * vec_dot(&v[k], &a[j]) / beta;
        for i in 0..n {
          a[j][i] -= gamma * v[k][i];
        }
      }
    }
    a.transpose(); // convert back
    let mut q = SquareMat::identity(n);
    let mut tmp = SquareMat::zeros(n);
    for i in 0..n - 1 {
      let v = &mut v[i];
      let v2 = v.iter().map(|x| *x * *x).sum::<f64>().sqrt();
      v.iter_mut().for_each(|x| *x /= v2);
      make_household(&mut tmp, v);
      q = &q * &tmp;
    }
    q
  }

  pub fn qr_method(a: &mut SquareMat) {
    for i in 1..100 {
      let q = qr(a);
      *a = &*a * &q;
      println!("iter {}:\n{:?}", i, a)
    }
  }

  pub fn solve() {
    let mut a = SquareMat::from_slice(&[
      0.5, 0.5, 0.5, 0.5,
      0.5, 0.5, -0.5, -0.5,
      0.5, -0.5, 0.5, -0.5,
      0.5, -0.5, -0.5, 0.5,
    ]);
    qr_method(&mut a);
  }
}

pub mod q4 {
  use crate::square_mat::{SquareMat, vec_dot};
  use super::q3::qr;

  pub fn shift_qr_method(a: &mut SquareMat, eps: f64) {
    let mut k = a.n();
    let mut iter = 0;
    while k > 0 && a[k][k - 1] > eps {
      iter += 1;
      let s = a[k][k];
      for j in 0..k {
        a[j][j] -= s;
      }
      let q = qr(a);
      *a = &*a * &q;
      for j in 0..k {
        a[j][j] -= s;
      }

      println!("iter {:?}\n{:?}", iter, a);
    }
  }

  pub fn solve() {
    let mut a = SquareMat::from_slice(&[
      0.5, 0.5, 0.5, 0.5,
      0.5, 0.5, -0.5, -0.5,
      0.5, -0.5, 0.5, -0.5,
      0.5, -0.5, -0.5, 0.5,
    ]);
    shift_qr_method(&mut a);
  }
}

//const A: [[f64; 4]; 4] = [[0.5, 0.5, 0.5, 0.5, ], [0.5, 0.5, -0.5, -0.5, ], [0.5, -0.5, 0.5, -0.5, ], [0.5, -0.5, -0.5, 0.5, ], ];