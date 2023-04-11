use crate::VectorOps;
use std::ops::{Add, AddAssign, Mul, Sub};

pub(crate) fn iter(length: usize) {
	for i in 0..length {
		//some operation
	}
}

impl<T> VectorOps<Vec<T>, T> for Vec<T>
where
	T: Add<T, Output = T> + Mul<T, Output = T> + Sub<T, Output = T> + AddAssign + Copy,
{
	type Output = T;

	fn vec_scal(&self, scal: T) -> Vec<T> {
		let mut result: Vec<T> = Vec::new();
		for i in 0..self.len() {
			result.push(self[i] * scal);
		}
		return result;
	}
	fn vec_add(&self, rhs: &Vec<T>) -> Vec<T> {
		if self.len() == 0 || rhs.len() == 0 {
			panic!("Cannot add a vector with zero elements");
		} else if self.len() != rhs.len() {
			panic!("Cant add two vectors with different length");
		}
		let mut result: Vec<T> = Vec::new();
		for i in 0..rhs.len() {
			result.push(self[i] + rhs[i])
		}
		return result;
	}

	fn dot(&self, rhs: &Vec<T>) -> T {
		if self.len() == 0 || rhs.len() == 0 {
			panic!("Trying to dot product with a zeroth scaler");
		} else if self.len() != rhs.len() {
			panic!("Cant dot product two vectors with different length");
		}
		let mut sum: Self::Output = rhs[0] - rhs[0];
		for i in 0..rhs.len() {
			sum += self[i] * rhs[i]
		}
		return sum;
	}
}

#[cfg(test)]
mod test_vec_ops {
	use crate::{Fsize, VectorOps};

	#[test]
	fn test_vec_add() {
		let vec_1 = vec![1.0, 1.0, 4.0];
		let vec_2 = vec![2.0, 3.0, 4.1];

		let vec_3 = vec![3.9, 3.9, 4.0, 3.1];
		let vec_4 = vec![4.0, 1.1, 4.5, 1.4];

		let vec_5 = vec![7.91, 8.23, 4.01, 6.70];
		let vec_6 = vec![0.73, 3.88, 1.42, 1.51];

		let vec_7 = vec![8.01, 0.93, 4.29, 6.12, 4.04, 7.19, 3.62, 5.21, 7.74];
		let vec_8 = vec![8.26, 6.12, 4.45, 1.31, 0.97, 5.87, 2.95, 1.30, 6.42];

		let test_fail: Vec<Fsize> = Vec::new();
		let test_fail_2 = vec![1.3, 4.4];
		let test_fail_3 = vec![1.3, 413.4, 14.1, 14.1];
		let result = std::panic::catch_unwind(|| test_fail_2.vec_add(&test_fail));
		let result_2 = std::panic::catch_unwind(|| test_fail.vec_add(&test_fail_2));
		let result_3 = std::panic::catch_unwind(|| test_fail_3.vec_add(&test_fail_2));
		let result_4 = std::panic::catch_unwind(|| test_fail_2.vec_add(&test_fail_3));

		assert_eq!(vec![3.0, 4.0, 8.1], vec_1.vec_add(&vec_2));
		assert_eq!(vec![7.9, 5.0, 8.5, 4.5], vec_3.vec_add(&vec_4));
		assert_eq!(vec![8.64, 12.11, 5.43, 8.21], vec_5.vec_add(&vec_6));
		assert_eq!(
			vec![16.27, 7.05, 8.74, 7.43, 5.01, 13.06, 6.57, 6.51, 14.16],
			vec_7.vec_add(&vec_8)
		);
		assert!(result.is_err());
		assert!(result_2.is_err());
		assert!(result_3.is_err());
		assert!(result_4.is_err());
	}
	#[test]
	fn test_scal_mult() {
		let vec_1 = vec![1.0, 1.0, 4.0];
		let vec_2 = vec![2.0, 3.0, 4.1];
		let vec_3 = vec![3.9, 3.9, 4.0, 3.1];
		let vec_4 = vec![4.0, 1.1, 4.5, 1.4];
		let vec_5 = vec![7.91, 8.23, 4.01, 6.70];
		let vec_6 = vec![0.73, 3.88, 1.42, 1.51];

		assert_eq!(vec![3.0, 3.0, 12.0], vec_1.vec_scal(3.0));
		assert_eq!(vec![8.0, 12.0, 16.4], vec_2.vec_scal(4.0));
		assert_eq!(vec![39.0, 39.0, 40.0, 31.0], vec_3.vec_scal(10.0));
		assert_eq!(vec![-4.0, -1.1, -4.5, -1.4], vec_4.vec_scal(-1.0));
		assert_eq!(
			vec![39.55, 41.150000000000006, 20.049999999999997, 33.5],
			vec_5.vec_scal(5.0)
		);
		assert_eq!(vec![-1.46, -7.76, -2.84, -3.02], vec_6.vec_scal(-2.0))
	}
	#[test]
	fn test_dot() {
		let vec_1 = vec![1.0, 2.0, 3.0];
		let vec_2 = vec![3.0, 4.0, 4.0];

		let vec_3 = vec![3.12, 4.0, 9.0, 1.12];
		let vec_4 = vec![-2.0, -1.0, 2.0, 3.0];

		let vec_5 = vec![1.2, 1.3];
		let vec_6 = vec![3.3, 3.3];

		let vec_7 = vec![1.0, 2.0, 1.0, 1.0, 5.0];
		let vec_8 = vec![3.0, 4.0, 4.9, 1.4, 4.1];

		let vec_9 = vec![5.0, -12.0];
		let vec_10 = vec![3.0, 4.0];

		let vec_11 = vec![4.0, 6.0, 7.0, 8.0];
		let vec_12 = vec![2.0, 1.0, 1.0, 2.0];

		let test_fail: Vec<Fsize> = Vec::new();
		let test_fail_2 = vec![11.4, 12.4];
		let test_fail_3 = vec![123.0, 3.6, 7.1, 56.144];
		let result = std::panic::catch_unwind(|| test_fail_2.vec_add(&test_fail));
		let result_2 = std::panic::catch_unwind(|| test_fail.vec_add(&test_fail_2));
		let result_3 = std::panic::catch_unwind(|| test_fail_3.vec_add(&test_fail_2));
		let result_4 = std::panic::catch_unwind(|| test_fail_2.vec_add(&test_fail_3));

		assert_eq!(23.0, vec_1.dot(&vec_2));
		assert_eq!(11.120000000000001, vec_3.dot(&vec_4));
		assert_eq!(8.25, vec_5.dot(&vec_6));
		assert_eq!(37.8, vec_7.dot(&vec_8));
		assert_eq!(-33.0, vec_9.dot(&vec_10));
		assert_eq!(37.0, vec_11.dot(&vec_12));
		assert!(result.is_err());
		assert!(result_2.is_err());
		assert!(result_3.is_err());
		assert!(result_4.is_err());
	}
}
