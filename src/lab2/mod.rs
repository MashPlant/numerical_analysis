pub mod q2 {
  fn damped_newton<V: Fn(f64) -> f64, D: Fn(f64) -> f64>(value: V, derivative: D, mut x0: f64, l0: f64, eps: f64) -> f64 {
    let mut iter = 0;
    loop {
      let mut v = value(x0);
      if v.abs() < eps {
        break x0;
      }
      let s = v / derivative(x0);
      let mut x1 = s;
      let mut l = l0;
      let mut v1 = value(x1);
      while v1.abs() >= v.abs() {
        v = v1;
        x1 = x0 - l * s;
        v1 = value(x1);
        l *= 0.5;
      }
      x0 = x1;
      eprintln!("iter{}: lambda={}, x={}", iter, l, x0);
      iter += 1;
    }
  }

  fn newton<V: Fn(f64) -> f64, D: Fn(f64) -> f64>(value: V, derivative: D, mut x0: f64, eps: f64) -> f64 {
    let mut iter = 0;
    loop {
      let mut v = value(x0);
      if v.abs() < eps {
        break x0;
      }
      x0 -= value(x0) / derivative(x0);
      eprintln!("iter{}: x={}", iter, x0);
      iter += 1;
    }
  }

  pub fn solve() {
    newton(|x| x * x * x - x - 1.0, |x| 3.0 * x * x - 1.0, 0.6, 1e-10);
    eprintln!();
    damped_newton(|x| x * x * x - x - 1.0, |x| 3.0 * x * x - 1.0, 0.6, 1.0, 1e-10);
    eprintln!();
    newton(|x| -x * x * x + 5.0 * x, |x| -3.0 * x * x + 5.0, 1.2, 1e-10);
    eprintln!();
    damped_newton(|x| -x * x * x + 5.0 * x, |x| -3.0 * x * x + 5.0, 1.2, 1.0, 1e-10);
    eprintln!();
//    damped_newton()
  }
}