extern crate gnuplot;
mod lab1;
mod lab2;
mod common;

fn main() {
  lab2::q2::solve();
//  let mut fg = Figure::new();
//  fg.axes2d()
//    .set_title("A plot", &[])
//    .set_legend(Graph(0.5), Graph(0.9), &[], &[])
//    .set_x_label("x", &[])
//    .set_y_label("y^2", &[])
//    .lines(
//      &[-3., -2., -1., 0., 1., 2., 3.],
//      &[9., 4., 1., 0., 1., 4., 9.],
//      &[Caption("Parabola")],
//    );
//  fg.show();
//  fg.echo_to_file("hello.png");
}
