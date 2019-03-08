#[derive(Copy, Clone)]
pub struct F64Iter {
  cur: f64,
  // exclusive
  end: f64,
  // for simplicity, positive only
  step: f64,
}

impl F64Iter {
  pub fn from_step(beg: f64, end: f64, step: f64) -> F64Iter {
    F64Iter { cur: beg, end, step }
  }

  pub fn from_n_step(beg: f64, end: f64, n_step: u32) -> F64Iter {
    F64Iter { cur: beg, end, step: (end - beg) / n_step as f64 }
  }
}

impl Iterator for F64Iter {
  type Item = f64;

  fn next(&mut self) -> Option<Self::Item> {
    if self.cur >= self.end { None }
    else {
      let ret = self.cur;
      self.cur += self.step;
      Some(ret)
    }
  }
}