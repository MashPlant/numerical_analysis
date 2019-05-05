pub mod q2 {
  fn damped_newton<V: Fn(f64) -> f64, D: Fn(f64) -> f64>(value: V, derivative: D, mut x0: f64, l0: f64, eps: f64) -> f64 {
    let mut iter = 0;
    loop {
      let v = value(x0);
      if v.abs() < eps {
        break x0;
      }
      let s = v / derivative(x0);
      let mut x1 = x0 - s;
      if value(x1).abs() < v.abs() {
        x0 = x1;
        eprintln!("iter{}: lambda unused, x={}", iter, x0);
        iter += 1;
        continue;
      }
      let mut l = l0;
      loop {
        x1 = x0 - l * s;
        if value(x1).abs() < v.abs() {
          break;
        }
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
      let v = value(x0);
      if v.abs() < eps {
        break x0;
      }
      x0 -= value(x0) / derivative(x0);
      eprintln!("iter{}: x={}", iter, x0);
      iter += 1;
    }
  }

  pub fn solve() {
    eprintln!("solving 3x^3-x-1=0, x0=0.6");

    eprintln!("damped newton method");
    damped_newton(|x| x * x * x - x - 1.0, |x| 3.0 * x * x - 1.0, 0.6, 0.95, 1e-10);
    eprintln!();

    eprintln!("newton method");
    newton(|x| x * x * x - x - 1.0, |x| 3.0 * x * x - 1.0, 0.6, 1e-10);
    eprintln!();

    eprintln!("fzero says x={}", super::q3::fzerotx(|x| x * x * x - x - 1.0, 0.0, 2.0, 1e-10));
    eprintln!();

    eprintln!("solving -x^3+5x=0, x0=1.35");

    eprintln!("damped newton method");
    damped_newton(|x| -x * x * x + 5.0 * x, |x| -3.0 * x * x + 5.0, 1.35, 0.95, 1e-10);
    eprintln!();

    eprintln!("newton method");
    newton(|x| -x * x * x + 5.0 * x, |x| -3.0 * x * x + 5.0, 1.35, 1e-10);
    eprintln!();

    eprintln!("fzero says x={}", super::q3::fzerotx(|x| -x * x * x + 5.0 * x, 1.0, 3.0, 1e-10));
    eprintln!();
  }
}

pub mod q3 {
  pub fn fzerotx<F: Fn(f64) -> f64>(f: F, mut a: f64, mut b: f64, eps: f64) -> f64 {
    let (mut fa, mut fb) = (f(a), f(b));
    assert_ne!(fa.signum(), fb.signum(), "function must change sign on the interval");
    let (mut c, mut fc) = (a, fa);
    let mut d = b - c;
    let mut e = d;
    loop {
      if fa.signum() == fb.signum() {
        (a = c, fa = fc, d = b - c, e = d);
      }
      if fa.abs() < fb.abs() {
        (c = b, b = a, a = c, fc = fb, fb = fa, fa = fc);
      }
      // convergence test and possible exit
      let m = 0.5 * (a - b);
      let tol = 2.0 * eps * b.abs().max(1.0);
      if m.abs() < tol || fb.abs() < eps {
        break b;
      }
      if e.abs() < tol || fc.abs() < fb.abs() { // bisection
        (d = m, e = m);
      } else { // interpolation
        let s = fb / fc;
        let mut p;
        let mut q;
        if a == c { // linear interpolation (secant)
          p = 2.0 * m * s;
          q = 1.0 - s;
        } else {
          q = fc / fa;
          let r = fb / fa;
          p = s * (2.0 * m * q * (q - r) - (b - c) * (r - 1.0));
          q = (q - 1.0) * (r - 1.0) * (s - 1.0);
        }
        if p > 0.0 { q = -q; } else { p = -p; }
        // is interpolated point acceptable
        if (2.0 * p < 3.0 * m * q - (tol * q).abs()) && (p < (0.5 * e * q).abs()) {
          (e = d, d = p / q);
        } else {
          (d = m, e = m);
        }
      }
      (c = b, fc = fb);
      if d.abs() > tol {
        b += d;
      } else {
        b -= (b - a).signum() * tol;
      }
      fb = f(b);
    }
  }

  pub fn solve() {
    use crate::special_fun::FloatSpecial;
    use crate::gnuplot::*;
    use crate::f64iter::*;
    use std::iter::repeat;
    const MAX: f64 = 40.0;
    let f = |x: f64| x.besselj(0.0);
    let mut ss = [0.0; 10];
    let mut cur = 0.0;
    for s in &mut ss {
      while f(cur).signum() == f(cur + 1.0).signum() {
        cur += 1.0;
      }
      *s = fzerotx(f, cur, cur + 1.0, 1e-15);
      cur += 1.0;
    }
    println!("{:?}", ss);
    println!("{:?}", ss.iter().map(|&x| f(x)).collect::<Vec<_>>());
    let mut fig = Figure::new();
    let xs = F64Iter::from_n_step(0.0, MAX, 10000);
    let axes = fig.axes2d()
      .set_title("f(x)=besselj(0, x)", &[])
      .arrow(Coordinate::Axis(0.0), Coordinate::Axis(0.0), Coordinate::Axis(MAX), Coordinate::Axis(0.0), &[])
      .lines(xs, xs.map(|x| x.besselj(0.0)), &[])
      .points(&ss, repeat(0.0f64), &[]);
    for &s in &ss {
      axes.label(&format!("{:.3}", s), Coordinate::Axis(s), Coordinate::Axis(-0.05), &[LabelOption::TextAlign(AlignCenter)]);
    }
    fig.show();
  }
}