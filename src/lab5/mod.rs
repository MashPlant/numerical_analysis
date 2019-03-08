pub mod q1 {
  use crate::square_mat::SquareMat;
  use crate::rand::{self, Rng};

  fn power_method(a: &SquareMat, eps: f64) -> (f64, Box<[f64]>) {
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
  pub fn solve() {

  }
}

pub mod q3 {
  pub fn solve() {

  }
}