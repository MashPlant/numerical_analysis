#![allow(dead_code)]

extern crate gnuplot;
extern crate special_fun;
extern crate rand;

mod lab1;
mod lab2;
mod lab3;
mod lab4;
mod lab5;
mod f64iter;
mod square_mat;
mod sparse_mat;

fn main() {
  lab5::q3::solve();
//  lab3::q6::solve();
//  let m = mat::SquareMat::all(10,1.0);
//  println!("{:?}", m);
//  lab2::q3::solve();
}
