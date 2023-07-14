use crate::{single_for_loop_operation, PointOperations};
use std::ops::{Add, Mul, Sub};

impl<T> PointOperations<Vec<T>, T> for Vec<T>
where
	T: Mul<T, Output = T> + Sub<T, Output = T> + Add<T, Output = T> + Copy,
{
	type Output = T;
	fn point_mult_scaler(&self, rhs: T) -> Vec<T> {
		if self.is_empty() {
			panic!("Cannot multiply a point of no dimensions");
		}
		let mut point: Vec<T> = Vec::with_capacity(self.len());
		let scal_mult_point_op = |index: usize| point.push(self[index] * rhs);
		single_for_loop_operation(self.len(), scal_mult_point_op);
		point
	}
	fn point_minus_point(&self, rhs: &Vec<T>) -> Vec<T> {
		if self.len() != rhs.len() {
			panic!("Dimensions of points do not match");
		} else if self.is_empty() || rhs.is_empty() {
			panic!("Point of zero dimensinality")
		}
		let mut point: Vec<T> = Vec::with_capacity(self.len());
		let point_minus_point_op = |index: usize| point.push(self[index] - rhs[index]);
		single_for_loop_operation(self.len(), point_minus_point_op);
		point
	}
	fn point_plus_vector(&self, rhs: &Vec<T>) -> Vec<T> {
		if self.len() != rhs.len() {
			panic!("The dimensions of the point and vector do not match")
		}
		let mut point: Vec<T> = Vec::with_capacity(self.len());
		let point_plut_vector_op = |index: usize| point.push(self[index] + rhs[index]);
		single_for_loop_operation(self.len(), point_plut_vector_op);
		point
	}
}

#[cfg(test)]
mod test_vec_point_rs {
	use crate::{Fsize, PointOperations};

	#[test]
	fn test_scaler_mult_point() {
		let point_a: Vec<Fsize> = vec![1.2, 2.3, 4.5];
		let point_b: Vec<Fsize> = vec![3.0, 14.0, 12.0];
		let point_c: Vec<Fsize> = vec![1.5, 6.5, 10.5];
		let point_d: Vec<Fsize> = vec![1.0, 1.0, 1.0, 1.0];
		let point_e: Vec<Fsize> = vec![];
		let result = std::panic::catch_unwind(|| point_e.point_mult_scaler(3.0));
		assert_eq!(
			vec![3.5999999999999996, 6.8999999999999995, 13.5],
			point_a.point_mult_scaler(3.0)
		);
		assert_eq!(vec![15.0, 70.0, 60.0], point_b.point_mult_scaler(5.0));
		assert_eq!(vec![3.0, 13.0, 21.0], point_c.point_mult_scaler(2.0));
		assert_eq!(
			vec![120.5, 120.5, 120.5, 120.5],
			point_d.point_mult_scaler(120.5)
		);
		assert!(result.is_err());
	}
	#[test]
	fn test_point_minus_point() {
		let point_a: Vec<Fsize> = vec![3.0, 2.0, 1.0];
		let point_b: Vec<Fsize> = vec![3.0, 2.0, 1.0];
		let point_c: Vec<Fsize> = vec![10.0, 10.0, 10.0];
		let point_d: Vec<Fsize> = vec![1.0, 1.0, 1.0];
		let point_e: Vec<Fsize> = vec![-2.5, -3.5, -4.5, -9.5];
		let point_f: Vec<Fsize> = vec![1.5, 1.5, 1.5, 1.5];

		let point_g: Vec<Fsize> = vec![1.0, 3.0, 4.0];
		let point_h: Vec<Fsize> = vec![1.0, 1.0, 3.0, 3.0];
		let point_i: Vec<Fsize> = vec![3.0, 4.0, 9.0, 1.9];
		let point_j: Vec<Fsize> = vec![];

		let result = std::panic::catch_unwind(|| point_g.point_minus_point(&point_h));
		let result_2 = std::panic::catch_unwind(|| point_i.point_minus_point(&point_j));

		assert_eq!(vec![0.0, 0.0, 0.0], point_a.point_minus_point(&point_b));
		assert_eq!(vec![9.0, 9.0, 9.0], point_c.point_minus_point(&point_d));
		assert_eq!(
			vec![-4.0, -5.0, -6.0, -11.0],
			point_e.point_minus_point(&point_f)
		);
		assert!(result.is_err());
		assert!(result_2.is_err());
	}
	#[test]
	fn test_point_plus_vector() {
		let point_a: Vec<Fsize> = vec![1.0, 2.0, 3.0, 0.0];
		let point_b: Vec<Fsize> = vec![3.5, 2.0, 1.0];
		let point_c: Vec<Fsize> = vec![0.5, 3.5, 1.5];
		let point_d: Vec<Fsize> = vec![2.4, 2.0, 1.4];
		let point_e: Vec<Fsize> = vec![1.0, 2.0, 0.0];
		let point_f: Vec<Fsize> = vec![3.0];

		let vec_a: Vec<Fsize> = vec![3.0, 4.0, 4.0, 4.0];
		let vec_b: Vec<Fsize> = vec![1.5, 4.0, 2.0];
		let vec_c: Vec<Fsize> = vec![0.5, 0.5, 0.5];
		let vec_d: Vec<Fsize> = vec![1.0, 1.0, 1.0];
		let vec_e: Vec<Fsize> = vec![0.0];
		let vec_f: Vec<Fsize> = vec![3.0, 4.0, 1.0];

		let result = std::panic::catch_unwind(|| point_e.point_plus_vector(&vec_e));
		let result_2 = std::panic::catch_unwind(|| point_f.point_plus_vector(&vec_f));

		assert_eq!(vec![4.0, 6.0, 7.0, 4.0], point_a.point_plus_vector(&vec_a));
		assert_eq!(vec![5.0, 6.0, 3.0], point_b.point_plus_vector(&vec_b));
		assert_eq!(vec![1.0, 4.0, 2.0], point_c.point_plus_vector(&vec_c));
		assert_eq!(vec![3.4, 3.0, 2.4], point_d.point_plus_vector(&vec_d));
		assert!(result.is_err());
		assert!(result_2.is_err());
	}
}
