use crate::TwoDimMatrixOps;
use num::Float;
use std::ops::{Add, Mul};

impl<T> TwoDimMatrixOps<T> for Vec<T>
where
	T: Mul<T, Output = T> + Add<T, Output = T> + Float + Copy + From<u8>,
{
	type Output = T;

	fn matrix_two_dim_rotate(&self, theta_rad: T, homogeneous_flag: bool) -> Vec<T> {
		_ = std::f32::consts::PI;
		if homogeneous_flag && self.len() != 3 {
			panic!("Homogeneous 2 dim vector is of size 3");
		} else if !homogeneous_flag && self.len() != 2 {
			panic!("Non homogeneous 2 dim vector is of size 2");
		}

		if homogeneous_flag {
			vec![
				self[0] * theta_rad.cos() - self[1] * theta_rad.sin(),
				self[0] * theta_rad.sin() + self[1] * theta_rad.cos(),
				self[2],
			]
		} else {
			vec![
				self[0] * theta_rad.cos() - self[1] * theta_rad.sin(),
				self[0] * theta_rad.sin() + self[1] * theta_rad.cos(),
			]
		}
	}
}

#[cfg(test)]
mod test_two_dim_matrix_ops {
	use crate::TwoDimMatrixOps;

	#[test]
	fn test_matrix_two_dim_rotate() {
		let matrix_a = vec![1.0, 0.0];
		assert_eq!(
			vec![-0.9999999999964793, 2.65358979335273e-6],
			matrix_a.matrix_two_dim_rotate(3.14159, false)
		);
		let matrix_b = vec![0.0, 1.0];
		assert_eq!(
			vec![-2.65358979335273e-6, -0.9999999999964793],
			matrix_b.matrix_two_dim_rotate(3.14159, false)
		);
		let matrix_c = vec![-1.0, 0.0, 1.0];
		assert_eq!(
			vec![0.9999999999964793, -2.65358979335273e-6, 1.0],
			matrix_c.matrix_two_dim_rotate(3.14159, true)
		);
		let matrix_d = vec![0.0, -1.0, 1.0];
		assert_eq!(
			vec![2.65358979335273e-6, 0.9999999999964793, 1.0],
			matrix_d.matrix_two_dim_rotate(3.14159, true)
		);
	}
}
