#![allow(dead_code)]
extern crate gnuplot;
extern crate special_fun;

mod lab1;
mod lab2;
mod lab3;
mod f64iter;
mod mat;

fn main() {
//  use std::iter::Rev;
//  for i in (0..100).rev() {
//    println!("{:?}", i);
//  }
//  use mat::SquareMat;
//  let a = SquareMat::all(1024, 0.1);
//  let b = SquareMat::all(1024, 0.01);
//  let _c = &a * &b;
//  println!("{:?}", _c);
  lab3::q6::solve();
//  let m = mat::SquareMat::all(10,1.0);
//  println!("{:?}", m);
//  lab2::q3::solve();
}
