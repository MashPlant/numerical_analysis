// implement example 1.4: truncation error & rounding error
pub mod q1 {
  use crate::gnuplot::*;
  use crate::common::*;

  pub fn solve() {
    const M: f64 = 1.0;
    const EPS: f64 = 1e-16;
    const N: u32 = 100;
    let truncation = |h: f64| M * h / 2.0;
    let rounding = |h: f64| 2.0 * EPS / h;
    let total = |h: f64| truncation(h) + rounding(h);
    let real = |h: f64| (((1.0 + h).sin() - 1.0f64.sin()) / h - 1.0f64.cos()).abs();
    let xs = F64Iter::from_n_step(-16.0, 0.0, N).map(|x| 10.0f64.powf(x)).collect::<Vec<_>>();
    let mut fig = Figure::new();
    fig.axes2d()
      .set_title("不同步长取值对应的差商近似导数的误差", &[])
      .lines(&xs, xs.iter().map(|&x| truncation(x)), &[Caption("截断误差"), Color("red")])
      .lines(&xs, xs.iter().map(|&x| rounding(x)), &[Caption("舍入误差"), Color("blue")])
      .lines(&xs, xs.iter().map(|&x| total(x)), &[Caption("总误差限"), Color("green")])
      .lines_points(xs.iter(), xs.iter().map(|&x| real(x)), &[Caption("实际总误差"), Color("black")])
      .set_x_log(Some(10.0))
      .set_y_log(Some(10.0));
    fig.show();
  }
}

// errors in calculating harmonic series
pub mod q3 {
  fn q1() -> (f32, i32) {
    let mut sum = 0.0;
    let mut n = 1;
    let n = loop {
      if sum + 1.0 / n as f32 == sum {
        break n;
      }
      sum += 1.0 / n as f32;
      n += 1;
    };
    println!("对于IEEE单精度浮点数，n={}时结果不再变化", n);
    (sum, n)
  }

  fn q2(float_res: f32, n: i32) {
    let double_res = {
      let mut sum = 0.0;
      for n in 1..n {
        sum += 1.0 / n as f64;
      }
      sum
    };
    println!("估计IEEE单精度浮点数计算结果的误差为{}", float_res as f64 - double_res);
  }

  fn q3() {
    use std::f64::EPSILON;
    // assume: sigma(1/n) = ln(n) + euler constant
    const E_MACH: f64 = EPSILON / 2.0;
    const EULER_CONSTANT: f64 = 0.57721566490153286060651209;
    let ratio = |x: f64| (1.0 / x) / (x.ln() + EULER_CONSTANT);
    let mut n = 1.0;
    loop {
      if ratio(n) < E_MACH {
        break;
      }
      n *= 2.0;
    }
    let n = n as u64;
    let (mut lo, mut hi) = (n / 2, n);
    while hi > lo + 1 {
      let mid = (lo + hi) / 2;
      if ratio(mid as f64) < E_MACH {
        hi = mid;
      } else {
        lo = mid;
      }
    }
    let n = hi;
    println!("如采用IEEE双精度浮点数，估计在n={}时求和结果不再变化，按照普通计算机每秒进行10^9次运算来估算，这需要{}s，即{:.3}天"
             , n, n / 1000000000, n as f64 / (1e9 * 3600.0 * 24.0));
  }

  pub fn solve() {
    let (float_res, n) = q1();
    q2(float_res, n);
    q3();
  }
}