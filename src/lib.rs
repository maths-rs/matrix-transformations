#[doc = include_str!("../README.md")]
mod matrix_operations;
pub mod vector_operations;

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

pub trait PointOperations<Rhs, Rhs2> {
	type Output;
	fn scaler_mult_point(&self, rhs: Rhs2) -> Vec<Self::Output>;
	fn point_minus_point(&self, rhs: Rhs) -> Vec<Self::Output>;
	fn point_plus_vector(&self) -> Vec<Self::Output>;
}
