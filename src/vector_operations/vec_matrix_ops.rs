use crate::{double_for_loop_operation, single_for_loop_operation, MatrixOperations};
use std::ops::{Add, AddAssign, Mul, Sub};

impl<T> MatrixOperations<Vec<Vec<T>>, Vec<T>, T> for Vec<Vec<T>>
where
	T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + AddAssign + Copy,
{
	type Output = T;
	fn matrix_add_matrix(&self, rhs: &Vec<Vec<T>>) -> Vec<Vec<T>> {
		if self.len() != rhs.len() {
			panic!("The rows dimensions do not match !");
		} else if self[0].len() != rhs[0].len() {
			panic!("The column dimension do not match")
		}
		let row_length: usize = rhs.len();
		let column_length: usize = rhs[0].len();

		let matrix_add_matrix_op = |row_vec: &mut Vec<T>, row: usize, column: usize| {
			row_vec.push(self[row][column] + rhs[row][column])
		};
		double_for_loop_operation(row_length, column_length, matrix_add_matrix)
	}
	fn matrix_mult_matrix(&self, rhs: &Vec<Vec<T>>) -> Vec<Vec<T>> {
		if self[0].len() != rhs.len() {
			panic!("Self matrix columns length does not match rhs row count")
		}

		let row_length = self.len();
		let column_length = rhs[0].len();

		let matrix_mult_matrix_op = |row_vec: &mut Vec<T>, row: usize, column: usize| {
			let mut temp = self[0][0] - self[0][0];
			for index in 0..column_length {
				temp += self[row][index] * rhs[index][column];
			}
			row_vec.push(temp);
		};
		double_for_loop_operation(row_length, column_length, matrix_add_matrix)
	}
	fn matrix_mult_point(&self, rhs: &Vec<T>) -> Vec<T> {
		if self[0].len() != rhs.len() {
			panic!("Self matrix columns length does not match rhs row count")
		}
		let mut point: Vec<T> = Vec::with_capacity(self[0].len());

		let matrix_mult_point_op = |row: usize| {
			let mut temp = self[0][0] - self[0][0];
			for column in 0..self[0].len() {
				temp += self[row][column] * rhs[column];
			}
			point.push(temp);
		};
		single_for_loop_operation(self.len(), dot_vec_op);
		point
	}
	fn scaler_mult_matrix(&self, rhs: T) -> Vec<Vec<T>> {
		let row_length: usize = self.len();
		let column_length: usize = self[0].len();

		let scaler_mult_matrix_op =
			|row_vec: &mut Vec<T>, row: usize, column: usize| row_vec.push(self[row][column] * rhs);
		double_for_loop_operation(row_length, column_length, matrix_add_matrix)
	}
}

#[cfg(test)]
mod test_overload {
	use crate::MatrixOperations;

	#[test]
	fn test_vec_matrix_add_matrix() {
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

		let matrix_c = vec![
			vec![1.5, 2.6, 1.1],
			vec![3.0, 4.0, 9.4],
			vec![4.4, 3.4, 3.4],
		];
		let matrix_d = vec![
			vec![4.4, 3.1, 2.3],
			vec![3.4, 15.0, 1.3],
			vec![4.5, 5.4, 1.4],
		];

		let matrix_e = vec![
			vec![9.8, 1.4, 1.3],
			vec![3.8, 1.9, 1.3],
			vec![9.3, 1.3, 9.8],
		];
		let matrix_f = vec![
			vec![19.91, 36.0, 14.98],
			vec![13.9, 198.3, 1.4],
			vec![3.0, 9.5, 3.8],
		];

		let matrix_g = vec![
			vec![8.03620, 0.31394, 7.00458, 2.21933],
			vec![8.12160, 4.59173, 4.09711, 1.55955],
			vec![8.73597, 6.77869, 2.08991, 3.70901],
			vec![7.25694, 2.85680, 7.46308, 1.18762],
		];

		let matrix_h = vec![
			vec![7.86803, 0.78678, 3.47480, 8.98299],
			vec![8.07704, 3.23203, 2.49040, 2.04194],
			vec![5.25651, 7.71115, 5.08027, 0.99102],
			vec![0.37106, 2.54352, 5.76318, 7.76062],
		];

		assert_eq!(
			vec![
				vec![3.0, 3.0, 4.0],
				vec![5.0, 5.0, 6.0],
				vec![6.0, 6.0, 7.0]
			],
			matrix_a.matrix_add_matrix(&matrix_b)
		);
		assert_eq!(
			vec![
				vec![5.9, 5.7, 3.4],
				vec![6.4, 19.0, 10.700000000000001],
				vec![8.9, 8.8, 4.8]
			],
			matrix_c.matrix_add_matrix(&matrix_d)
		);
		assert_eq!(
			vec![
				vec![29.71, 37.4, 16.28],
				vec![17.7, 200.20000000000002, 2.7],
				vec![12.3, 10.8, 13.600000000000001]
			],
			matrix_e.matrix_add_matrix(&matrix_f)
		);
		assert_eq!(
			vec![
				vec![
					15.904229999999998,
					1.10072,
					10.479379999999999,
					11.202319999999999,
				],
				vec![16.19864, 7.82376, 6.58751, 3.60149],
				vec![13.99248, 14.489840000000001, 7.17018, 4.70003],
				vec![7.628, 5.40032, 13.22626, 8.94824]
			],
			matrix_g.matrix_add_matrix(&matrix_h)
		)
	}
	#[test]
	fn test_vec_matrix_mult_matrix() {
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

		let matrix_c = vec![
			vec![0.59069, 6.73303, 1.04537],
			vec![3.03173, 7.47636, 3.14113],
			vec![2.32089, 0.75388, 3.97486],
		];
		let matrix_d = vec![
			vec![1.69346, 7.97014, 4.50808],
			vec![5.60360, 6.77020, 0.65242],
			vec![6.99582, 6.07495, 5.65828],
		];

		let matrix_e = vec![
			vec![3.39837, 5.62793, 3.29381, 2.99721],
			vec![6.34436, 5.37185, 8.40999, 7.10067],
			vec![4.23070, 2.97570, 8.71890, 0.25334],
			vec![1.99635, 1.06084, 6.43337, 4.76202],
		];

		let matrix_f = vec![
			vec![0.66250, 1.92013, 1.53517, 0.22483],
			vec![4.78176, 4.50655, 4.49441, 5.65035],
			vec![2.64023, 4.77948, 1.31993, 3.51463],
			vec![2.28443, 4.02282, 0.60577, 2.79788],
		];

		assert_eq!(
			vec![
				vec![11.0, 11.0, 14.0],
				vec![11.0, 11.0, 14.0],
				vec![11.0, 11.0, 14.0]
			],
			matrix_a.matrix_mult_matrix(&matrix_b)
		);

		assert_eq!(
			vec![
				vec![46.0427371488, 56.6424121841, 12.970637371399999],
				vec![69.0034244584, 93.86197270769999, 36.318401226],
				vec![35.962181432600005, 47.7488123576, 33.445475021600004]
			],
			matrix_c.matrix_mult_matrix(&matrix_d)
		);

		assert_eq!(
			vec![
				vec![
					44.706163098400005,
					59.6877954806,
					36.6745190792,
					52.52618715770001,
				],
				vec![68.3153274218, 105.1506028789, 49.2849384663, 81.2040608396],
				vec![40.6305608252, 64.2245842168, 31.5306630048, 49.1174572022],
				vec![
					34.2593419771,
					58.518892571500004,
					19.208873453400003,
					42.3774323852,
				]
			],
			matrix_e.matrix_mult_matrix(&matrix_f)
		);
	}
	#[test]
	fn test_vec_scaler_mult_matrix() {
		let matrix_a = vec![
			vec![1.0, 1.0, 1.0],
			vec![1.0, 1.0, 1.0],
			vec![1.0, 1.0, 1.0],
		];
		let matrix_b = vec![
			vec![1.0, 2.0, 3.0],
			vec![4.0, 5.0, 6.0],
			vec![7.0, 8.0, 9.0],
		];
		let matrix_c = vec![vec![10.0, 15.0, 20.0]];

		let matrix_d = vec![
			vec![15.5, 1.9, 2.9],
			vec![1.4, 1.5, 15.5],
			vec![1.4, 4.5, 14.4],
		];
		let matrix_e = vec![
			vec![15.5, 1.9, 2.9, 12.3, 55.1],
			vec![1.4, 1.5, 15.5, 14.13, 134.14],
		];

		assert_eq!(
			vec![
				vec![3.0, 3.0, 3.0],
				vec![3.0, 3.0, 3.0],
				vec![3.0, 3.0, 3.0],
			],
			matrix_a.scaler_mult_matrix(3.0)
		);
		assert_eq!(
			vec![
				vec![3.0, 6.0, 9.0],
				vec![12.0, 15.0, 18.0],
				vec![21.0, 24.0, 27.0],
			],
			matrix_b.scaler_mult_matrix(3.0)
		);
		assert_eq!(
			vec![vec![50.0, 75.0, 100.0],],
			matrix_c.scaler_mult_matrix(5.0)
		);
		assert_eq!(
			vec![
				vec![108.5, 13.299999999999999, 20.3],
				vec![9.799999999999999, 10.5, 108.5],
				vec![9.799999999999999, 31.5, 100.8]
			],
			matrix_d.scaler_mult_matrix(7.0)
		);
		//vec![15.5, 1.9, 2.9, 12.3, 55.1],
		//vec![1.4, 1.5, 15.5, 14.13, 134.14],
		assert_eq!(
			vec![
				vec![34.1, 4.18, 6.38, 27.060000000000002, 121.22000000000001],
				vec![3.08, 3.3000000000000003, 34.1, 31.086000000000006, 295.108]
			],
			matrix_e.scaler_mult_matrix(2.2)
		);
	}
	#[test]
	fn test_vec_matrix_mult_point() {
		let matrix_a = vec![
			vec![3.0, 4.0, 6.0],
			vec![2.5, 7.0, 8.0],
			vec![9.0, 22.0, 45.0],
		];
		let point_a = vec![7.0, 6.0, 21.0];

		let matrix_b = vec![vec![7.0, 6.0, 4.0, 2.0], vec![1.0, 2.0, 1.0, 2.0]];
		let point_b = vec![6.0, 4.0, 1.0, 2.0];

		let matrix_c = vec![vec![6.0, 2.5], vec![3.8, 7.9]];
		let point_c = vec![8.0, 9.5];

		assert_eq!(
			vec![171.0, 227.5, 1140.0],
			matrix_a.matrix_mult_point(&point_a)
		);
		assert_eq!(vec![74.0, 19.0], matrix_b.matrix_mult_point(&point_b));
		assert_eq!(
			vec![71.75, 105.44999999999999],
			matrix_c.matrix_mult_point(&point_c)
		)
	}
}
