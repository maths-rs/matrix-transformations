use crate::{single_for_loop_operation, VecScalingProjection, VectorOps};
use num::Float;
use std::ops::{AddAssign, Mul, SubAssign};

impl<T> VecScalingProjection<Vec<T>> for Vec<T>
where
	T: AddAssign + SubAssign + Mul<T, Output = T> + Float + std::fmt::Debug,
{
	type Output = T;

	#[allow(clippy::eq_op)]
	fn magnitude(&self) -> T {
		if self.is_empty() {
			panic!("Cannot get length of a length zero vector")
		}
		let mut result: T = self[0] - self[0];
		let magnitude_vec_op = |index: usize| result += self[index] * self[index];
		single_for_loop_operation(self.len(), magnitude_vec_op);
		result.sqrt()
	}
	fn vec_scalar_components(&self, rhs: &Vec<T>) -> T {
		self.dot(rhs) / rhs.magnitude()
	}
	fn vec_projection(&self, rhs: &Vec<T>) -> Vec<T> {
		let rhs_mag: T = rhs.magnitude();
		let scaler: T = self.dot(rhs) / (rhs_mag * rhs_mag);
		rhs.vec_scal(scaler)
	}
}

#[cfg(test)]
mod test_vec_ops {
	use crate::{Fsize, VecScalingProjection};
	#[test]
	fn test_magnitude() {
		let vec_1 = vec![1.0, 2.0, 3.0, 4.0];
		let vec_2 = vec![3.0, 1.3, 4.1];
		let vec_3 = vec![1.4, 1.4, 14.5, 14.4, 2.0];
		let vec_4 = vec![0.0, 0.0, 0.0, 0.0, 0.0];
		let vec_5 = vec![5.0, -12.0];
		let vec_6 = vec![2.0, 1.0, 1.0, 2.0];

		let empty: Vec<Fsize> = Vec::new();
		let result = std::panic::catch_unwind(|| empty.magnitude());

		assert_eq!((30.0 as Fsize).sqrt(), vec_1.magnitude());
		assert_eq!((27.5 as Fsize).sqrt(), vec_2.magnitude());
		assert_eq!((425.53 as Fsize).sqrt(), vec_3.magnitude());
		assert_eq!((0.0 as Fsize).sqrt(), vec_4.magnitude());
		assert_eq!((169.0 as Fsize).sqrt(), vec_5.magnitude());
		assert_eq!((10.0 as Fsize).sqrt(), vec_6.magnitude());
		assert!(result.is_err());
	}
	#[test]
	fn test_scalar_components() {
		let vec_1 = vec![1.0, 3.0];
		let vec_2 = vec![2.0, 1.0];

		let vec_3 = vec![3.0, 4.0];
		let vec_4 = vec![5.0, -12.0];

		let vec_5 = vec![4.0, 6.0, 7.0, 8.0];
		let vec_6 = vec![2.0, 1.0, 1.0, 2.0];

		assert_eq!((5.0 as Fsize).sqrt(), vec_1.vec_scalar_components(&vec_2));
		assert_eq!((-33.0 / 13.0 as Fsize), vec_3.vec_scalar_components(&vec_4));
		assert_eq!(
			((37.0 as Fsize) / ((10.0 as Fsize).sqrt())),
			vec_5.vec_scalar_components(&vec_6)
		);
	}
	#[test]
	fn test_vector_projection() {
		let vec_1 = vec![1.0, 3.0];
		let vec_2 = vec![2.0, 1.0];

		let vec_3 = vec![1.0, 2.0];
		let vec_4 = vec![-3.0, 4.0];

		let vec_5 = vec![-1.0, 4.0, 2.0];
		let vec_6 = vec![1.0, 0.0, 3.0];

		assert_eq!(
			vec![1.9999999999999996, 0.9999999999999998],
			vec_1.vec_projection(&vec_2)
		);
		assert_eq!(
			vec![-0.6000000000000001, 4.0 / 5.0],
			vec_3.vec_projection(&vec_4)
		);
		assert_eq!(
			vec![0.4999999999999999, 0.0, 1.4999999999999996],
			vec_5.vec_projection(&vec_6)
		);
	}
}
