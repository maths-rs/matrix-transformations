use crate::PointOperations;
use std::ops::{Add, AddAssign, Mul, Sub};

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
	fn point_minus_point(&self, rhs: Vec<T>) -> Vec<T> {
		let mut point: Vec<T> = Vec::with_capacity(self.len());
		for i in 0..self.len() {
			//point.push(self[i] * rhs);
		}
		return point;
	}
	fn point_plus_vector(&self) -> Vec<T> {
		let mut point: Vec<T> = Vec::with_capacity(self.len());
		for i in 0..self.len() {
			//point.push(self[i] * rhs);
		}
		return point;
	}
}
