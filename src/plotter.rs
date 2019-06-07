use gnuplot::*;
use ndarray::*;
use spaces::{Matrix};

pub fn plot_points(pts: Matrix) {
	let (x_pts, y_pts) = get_points_for_plot(pts);
	let mut fg = Figure::new();
	fg.axes2d()
		.set_title("Agent locations", &[])
		.set_legend(Graph(0.0), Graph(1.0), &[], &[])
		.set_x_label("x pos", &[])
		.set_y_label("y pos", &[])
		.points(
			&x_pts,
			&y_pts,
			&[Caption("Agent points")],
		);
	fg.show();

}

pub fn get_points_for_plot(pts: Matrix) -> (Vec<f64>, Vec<f64>) {
	let (mut x_pts, mut y_pts) = (vec![], vec![]);
	if pts.row(0).len() == 1 {
		x_pts = pts.column(0).to_vec();
		y_pts = Array::zeros(x_pts.len()).to_vec();
	} else if pts.row(0).len() == 2 {
		x_pts = pts.column(0).to_vec();
		y_pts = pts.column(1).to_vec();
	}

	(x_pts, y_pts)
}