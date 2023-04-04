use crate::{MatrixOperations, PointOperations};
use std::ops::{Add, AddAssign, Mul, Sub};

impl<T> MatrixOperations<Vec<Vec<T>>, Vec<T>, T> for Vec<Vec<T>>
where
	T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + AddAssign + Copy,
{
	type Output = T;
	fn add_matrix(&self, rhs: &Vec<Vec<T>>) -> Vec<Vec<T>> {
		let row_length: usize = rhs.len();
		let column_length: usize = rhs[0].len();

		let mut matrix: Vec<Vec<T>> = Vec::with_capacity(row_length);
		for row in 0..row_length {
			let mut vec_row: Vec<T> = Vec::with_capacity(column_length);
			for column in 0..column_length {
				vec_row.push(self[row][column] + rhs[row][column])
			}
			matrix.push(vec_row);
		}
		return matrix;
	}
	fn matrix_mult_matrix(&self, rhs: &Vec<Vec<T>>) -> Vec<Vec<T>> {
		if self[0].len() != rhs.len() {
			panic!("Self matrix columns length does not match rhs row count")
		}

		let rows_length = self.len();
		let column_length = rhs[0].len();
		let mut matrix: Vec<Vec<T>> = Vec::with_capacity(rows_length);
		for row in 0..rows_length {
			let mut row_vec: Vec<T> = Vec::with_capacity(self.len());
			for column in 0..column_length {
				let mut temp = self[0][0] - self[0][0];
				for index in 0..column_length {
					temp += self[row][index] * rhs[index][column];
				}
				row_vec.push(temp);
			}
			matrix.push(row_vec);
		}
		matrix
	}
	fn matrix_mult_point(&self, rhs: &Vec<T>) -> Vec<T> {
		if self[0].len() != rhs.len() {
			panic!("Self matrix columns length does not match rhs row count")
		}
		let mut point: Vec<T> = Vec::with_capacity(self[0].len());
		for row in 0..self.len() {
			let mut temp = self[0][0] - self[0][0];
			for column in 0..self[0].len() {
				temp += self[row][column] * rhs[column];
			}
			point.push(temp);
		}
		point
	}
	fn scaler_mult_matrix(&self, rhs: T) -> Vec<Vec<T>> {
		let row_length: usize = self.len();
		let column_length: usize = self[0].len();
		let mut matrix: Vec<Vec<T>> = Vec::with_capacity(row_length);
		for row in 0..row_length {
			let mut vec_row: Vec<T> = Vec::with_capacity(column_length);
			for column in 0..column_length {
				vec_row.push(self[row][column] * rhs);
			}
			matrix.push(vec_row);
		}
		matrix
	}
}

impl<T> PointOperations<Vec<T>, T> for Vec<T>
where
	T: Mul<T, Output = T> + Copy,
{
	type Output = T;
	fn scaler_mult_point(&self, rhs: T) -> Vec<T> {
		let mut point: Vec<T> = Vec::with_capacity(self.len());
		for i in 0..self.len() {
			point.push(self[i] * rhs);
		}
		return point;
	}
}

#[cfg(test)]
mod test_overload {
	use crate::MatrixOperations;

	#[test]
	fn test_add_vec_matrix() {
		let matrix_a = vec![
			vec![1.0, 1.0, 1.0],
			vec![1.0, 1.0, 1.0],
			vec![1.0, 1.0, 1.0],
		];
		let matrix_b = vec![
			vec![2.0, 2.0, 3.0],
			vec![4.0, 4.0, 5.0],
			vec![5.0, 5.0, 6.0],
		];

		assert_eq!(
			vec![
				vec![3.0, 3.0, 4.0],
				vec![5.0, 5.0, 6.0],
				vec![6.0, 6.0, 7.0]
			],
			matrix_a.add_matrix(&matrix_b)
		);
	}
}
