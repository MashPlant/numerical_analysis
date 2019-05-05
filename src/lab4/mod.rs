pub mod q2 {
  use crate::sparse_mat::SparseMat;
  use crate::f64iter::F64Iter;
  use crate::gnuplot::*;
  use std::iter::repeat;
  use crate::square_mat::vec_dis_inf;

  fn jacobi(a: &SparseMat, b: &[f64], x: &mut [f64], eps: f64) -> u32 {
    assert_eq!(a.n(), b.len());
    assert_eq!(a.n(), x.len());
    let n = a.n();
    let mut y: Box<[f64]> = Box::from(&*x);
    let mut iter = 0;
    loop {
      let mut can_break = true;
      iter += 1;
      y.clone_from_slice(x);
      for i in 0..n {
        let mut sum = 0.0;
        for &(a, col) in a.row_at(i) {
          sum += a * y[col];
        }
        let old = x[i];
        x[i] = (b[i] - sum) / a.diagonal_at(i);
        let diff = (old - x[i]).abs();
        if old.abs() * eps < diff { can_break = false; }
      }
      if can_break { break iter; }
    }
  }

  fn gs(a: &SparseMat, b: &[f64], x: &mut [f64], eps: f64) -> u32 {
    assert_eq!(a.n(), b.len());
    assert_eq!(a.n(), x.len());
    let n = a.n();
    let mut iter = 0;
    loop {
      let mut can_break = true;
      iter += 1;
      for i in 0..n {
        let mut sum = 0.0;
        for &(a, col) in a.row_at(i) {
          sum += a * x[col];
        }
        let old = x[i];
        x[i] = (b[i] - sum) / a.diagonal_at(i);
        let diff = (old - x[i]).abs();
        if old.abs() * eps < diff { can_break = false; }
      }
      if can_break { break iter; }
    }
  }

  fn sor(a: &SparseMat, b: &[f64], x: &mut [f64], eps: f64, w: f64) -> u32 {
    assert_eq!(a.n(), b.len());
    assert_eq!(a.n(), x.len());
    let n = a.n();
    let mut iter = 0;
    loop {
      let mut can_break = true;
      iter += 1;
      for i in 0..n {
        let mut sum = 0.0;
        for &(a, col) in a.row_at(i) {
          sum += a * x[col];
        }
        let old = x[i];
        x[i] = (1.0 - w) * x[i] + w * (b[i] - sum) / a.diagonal_at(i);
        let diff = (old - x[i]).abs();
        if old.abs() * eps < diff { can_break = false; }
      }
      if can_break { break iter; }
    }
  }

  pub fn tmp() {
    let mut a = SparseMat::from_diagonal(vec![5.0, 4.0, 10.0].into_iter());
    a.add(0, 1, 2.0);
    a.add(0, 2, 1.0);
    a.add(1, 0, -1.0);
    a.add(1, 2, 2.0);
    a.add(2, 0, 2.0);
    a.add(2, 1, -3.0);
    sor(&a, &[-12.0, 20.0, 3.0], &mut [0.0, 0.0, 0.0], 1e-3, 0.9);
  }

  fn make_a(eps: f64, n: usize, h: f64) -> SparseMat {
    let mut ret = SparseMat::from_diagonal(repeat(-(2.0 * eps + h)).take(n));
    for i in 0..n - 1 {
      ret.add(i, i + 1, eps + h);
      ret.add(i + 1, i, eps);
    }
    ret
  }

  pub fn solve() {
    const A: f64 = 0.5;
    const N: usize = 100;
    const H: f64 = 1.0 / N as f64;
    let mut b = [A * H * H; N - 1];
    for (init_kind, init) in [repeat(0.0).take(N - 1).collect::<Box<[_]>>(),
      F64Iter::from_step(H, 1.0, H).collect::<Box<[_]>>()].iter().enumerate() {
      for &eps in &[1.0, 0.1, 0.01, 0.0001] {
        println!("eps = {}", eps);
        // no need to change b[0], y0 is missed, but y0 = 0, so b[0] will not change
        b[N - 2] = A * H * H - (eps + H); // miss yn = 1.0
        let a = make_a(eps, N - 1, H);
        let acc = F64Iter::from_step(H, 1.0, H).map(|x|
          (1.0 - A) / (1.0 - (-1.0 / eps).exp()) * (1.0 - (-x / eps).exp()) + A * x).collect::<Box<[_]>>();
        let mut gauss = b.iter().map(|&x| x).collect::<Box<_>>();
        crate::lab3::q6::gauss(&mut a.to_dense(), gauss.as_mut());

        macro_rules! plot {
          ($solve: expr, $title: expr) => {
            let mut fig = Figure::new();
            let xs = F64Iter::from_step(H, 1.0, H);
            fig.axes2d()
              .set_title(&format!("method = {}, n = {}, a = {}, eps = {}", $title, N, A, eps), &[])
              .lines(xs, acc.as_ref(), &[])
              .lines_points(xs, $solve.as_ref(), &[])
              .lines_points(xs, gauss.as_ref(), &[]);
            fig.show();
          };
        }

        let mut x = init.clone();
        let iter = jacobi(&a, &b, &mut x, 1e-5);
        println!("jacobi: iter = {}, inf norm dist = {}", iter, vec_dis_inf(&x, &gauss));
        if init_kind == 0 {
          plot!(x, "jacobi");
        }

        x.clone_from_slice(init);
        let iter = gs(&a, &b, &mut x, 1e-5);
        println!("gs: iter = {}, inf norm dist = {}", iter, vec_dis_inf(&x, &gauss));
//        plot!(x, "gs");

        x.clone_from_slice(init);
        let iter = sor(&a, &b, &mut x, 1e-5, 1.1);
        println!("sor(w = {}): iter = {}, inf norm dist = {}", 1.1, iter, vec_dis_inf(&x, &gauss));
//        plot!(x, &format!("sor(w = {})", 1.1));

        println!();
      }
    }
  }
}