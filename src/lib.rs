#[doc = include_str!("../README.md")]
pub mod vector_operations;
use std::ops::{Add, AddAssign, Mul, Sub};

#[cfg(target_pointer_width = "64")]
pub type Fsize = f64;
#[cfg(target_pointer_width = "32")]
pub type Fsize = f32;

pub type VectorSD = Vec<Fsize>;
pub type VectorMD = Vec<Vec<Fsize>>;

pub type Matrix3D = [[Fsize; 4]; 4];
pub type Matrix2D = [[Fsize; 3]; 3];

pub type Point3D = [Fsize; 4];
pub type Point2D = [Fsize; 3];

pub const I4: Matrix3D = [
	[1.0, 0.0, 0.0, 0.0],
	[0.0, 1.0, 0.0, 0.0],
	[0.0, 0.0, 1.0, 0.0],
	[0.0, 0.0, 0.0, 1.0],
];

pub const I3: Matrix2D = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

pub trait VectorOps<Rhs, A> {
	type Output;
	fn vec_scal(&self, scal: A) -> Vec<Self::Output>;
	fn vec_add(&self, rhs: &Rhs) -> Vec<Self::Output>;
	fn dot(&self, rhs: &Rhs) -> Self::Output;
}

pub trait VecScalingProjection<Rhs> {
	type Output;
	fn magnitude(&self) -> Self::Output;
	fn vec_scalar_components(&self, rhs: &Rhs) -> Self::Output;
	fn vec_projection(&self, rhs: &Rhs) -> Vec<Self::Output>;
}

pub trait MatrixOperations<Rhs, Rhs2, Rhs3> {
	type Output;
	fn matrix_add_matrix(&self, rhs: &Rhs) -> Vec<Vec<Self::Output>>;
	fn matrix_mult_matrix(&self, rhs: &Rhs) -> Vec<Vec<Self::Output>>;
	fn matrix_mult_point(&self, rhs: &Rhs2) -> Vec<Self::Output>;
	fn scaler_mult_matrix(&self, rhs: Rhs3) -> Vec<Vec<Self::Output>>;
}

pub trait TwoDimMatrixOps<Rhs> {
	type Output;
	fn matrix_two_dim_rotate(&self, wheta: Rhs, homogeneous_flag: bool) -> Vec<Self::Output>;
}

pub trait PointOperations<Rhs, Rhs2> {
	type Output;
	fn point_mult_scaler(&self, rhs: Rhs2) -> Vec<Self::Output>;
	fn point_minus_point(&self, rhs: &Rhs) -> Vec<Self::Output>;
	fn point_plus_vector(&self, rhs: &Rhs) -> Vec<Self::Output>;
}

//for loop used to perform operations which only require one for loop
pub(crate) fn single_for_loop_operation<F>(length: usize, mut operation_function: F)
where
	F: FnMut(usize),
{
	for i in 0..length {
		operation_function(i);
	}
}

//for loop used to perform operations which require two for loops
pub(crate) fn double_for_loop_operation<F, T>(
	row_length: usize,
	column_length: usize,
	operation_function: F,
) -> Vec<Vec<T>>
where
	F: Fn(&mut Vec<T>, usize, usize),
	T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + AddAssign + Copy,
{
	let mut matrix: Vec<Vec<T>> = Vec::with_capacity(row_length);
	for row in 0..row_length {
		let mut vec_row: Vec<T> = Vec::with_capacity(column_length);
		for column in 0..column_length {
			operation_function(&mut vec_row, row, column);
		}
		matrix.push(vec_row);
	}
	matrix
}
